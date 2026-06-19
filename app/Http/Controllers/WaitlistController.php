<?php

namespace App\Http\Controllers;

use App\Mail\WaitlistWelcomeMail;
use App\Models\WaitlistSubscriber;
use Illuminate\Http\RedirectResponse;
use Illuminate\Http\Request;
use Illuminate\Support\Facades\Log;
use Illuminate\Support\Facades\Mail;
use Illuminate\Validation\Rule;
use Inertia\Inertia;
use Inertia\Response;

class WaitlistController extends Controller
{
    public function index(): Response
    {
        return Inertia::render('Waitlist', [
            'subscribers' => WaitlistSubscriber::latest()
                ->take(200)
                ->get(['id', 'email', 'platform', 'created_at']),
            'stats' => [
                'total' => WaitlistSubscriber::count(),
                'mac' => WaitlistSubscriber::where('platform', 'mac')->count(),
                'windows' => WaitlistSubscriber::where('platform', 'windows')->count(),
                'unknown' => WaitlistSubscriber::whereNull('platform')->count(),
            ],
        ]);
    }

    public function store(Request $request): RedirectResponse
    {
        $validated = $request->validate([
            'email' => ['required', 'email', 'max:255'],
            'platform' => ['nullable', Rule::in(['mac', 'windows'])],
        ]);

        $subscriber = WaitlistSubscriber::firstOrCreate(
            ['email' => $validated['email']],
            ['platform' => $validated['platform'] ?? null],
        );

        if ($subscriber->wasRecentlyCreated) {
            try {
                Mail::to($subscriber->email)
                    ->send(new WaitlistWelcomeMail($subscriber->platform));
            } catch (\Throwable $e) {
                // Never fail the signup because the welcome email could not be sent.
                Log::warning('Waitlist welcome email failed', [
                    'email' => $subscriber->email,
                    'error' => $e->getMessage(),
                ]);
            }
        }

        return back()->with('success', 'You are on the list. We will email you when ChromaVale is ready to download.');
    }
}
