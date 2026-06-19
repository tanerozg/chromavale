<?php

namespace App\Http\Controllers;

use App\Models\WaitlistSubscriber;
use Illuminate\Http\RedirectResponse;
use Illuminate\Http\Request;
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

        WaitlistSubscriber::firstOrCreate(
            ['email' => $validated['email']],
            ['platform' => $validated['platform'] ?? null],
        );

        return back()->with('success', 'You are on the list. We will email you when ChromaVale is ready to download.');
    }
}
