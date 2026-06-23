<?php

namespace App\Http\Controllers;

use App\Mail\LicenseKeyMail;
use App\Models\License;
use Illuminate\Http\RedirectResponse;
use Illuminate\Http\Request;
use Illuminate\Http\Response;
use Illuminate\Support\Facades\Log;
use Illuminate\Support\Facades\Mail;
use Stripe\Checkout\Session as CheckoutSession;
use Stripe\Stripe;
use Stripe\Webhook;

class CheckoutController extends Controller
{
    /**
     * Create a Stripe Checkout session for the one-time Pro purchase and
     * redirect the buyer to Stripe. With Stripe Managed Payments enabled on
     * the account, Stripe acts as merchant of record and handles tax.
     */
    public function pro(): RedirectResponse
    {
        $secret = config('services.stripe.secret');
        $price = config('services.stripe.pro_price_id');

        if (! $secret || ! $price) {
            return back()->with('error', 'Checkout is not configured yet.');
        }

        Stripe::setApiKey($secret);

        $session = CheckoutSession::create([
            'mode' => 'payment',
            'line_items' => [[
                'price' => $price,
                'quantity' => 1,
            ]],
            'allow_promotion_codes' => true,
            'success_url' => route('pro.thanks').'?session_id={CHECKOUT_SESSION_ID}',
            'cancel_url' => url('/#pricing'),
        ]);

        return redirect()->away($session->url);
    }

    /**
     * Stripe webhook: issue and email a license when a purchase completes.
     */
    public function webhook(Request $request): Response
    {
        $secret = config('services.stripe.webhook_secret');

        try {
            $event = Webhook::constructEvent(
                $request->getContent(),
                $request->header('Stripe-Signature'),
                $secret,
            );
        } catch (\Throwable $e) {
            return response('Invalid signature', 400);
        }

        if ($event->type === 'checkout.session.completed') {
            $session = $event->data->object;

            // Idempotent: one license per checkout session.
            if (! License::where('stripe_session_id', $session->id)->exists()) {
                $email = $session->customer_details->email
                    ?? $session->customer_email
                    ?? 'unknown@chromavale.com';

                $license = License::create([
                    'key' => License::generateKey(),
                    'email' => $email,
                    'status' => 'active',
                    'stripe_session_id' => $session->id,
                    'stripe_payment_intent' => $session->payment_intent ?? null,
                    'devices' => [],
                ]);

                try {
                    Mail::to($email)->send(new LicenseKeyMail($license->key));
                } catch (\Throwable $e) {
                    Log::warning('License email failed', [
                        'email' => $email,
                        'error' => $e->getMessage(),
                    ]);
                }
            }
        }

        return response('ok', 200);
    }
}
