<?php

namespace App\Http\Controllers;

use App\Models\License;
use Illuminate\Http\JsonResponse;
use Illuminate\Http\Request;

class LicenseController extends Controller
{
    /**
     * Activate / validate a license for a device. Called by the desktop app.
     *
     * Request: { key, device_id }
     * Response: { valid: bool, email?: string, reason?: string }
     */
    public function activate(Request $request): JsonResponse
    {
        $validated = $request->validate([
            'key' => ['required', 'string', 'max:64'],
            'device_id' => ['required', 'string', 'max:128'],
        ]);

        $license = License::where('key', strtoupper(trim($validated['key'])))->first();

        if (! $license) {
            return response()->json(['valid' => false, 'reason' => 'unknown'], 404);
        }

        if (! $license->isActive()) {
            return response()->json(['valid' => false, 'reason' => 'inactive'], 403);
        }

        if (! $license->activateDevice($validated['device_id'])) {
            return response()->json(['valid' => false, 'reason' => 'device_limit'], 403);
        }

        return response()->json([
            'valid' => true,
            'email' => $license->email,
            'plan' => 'pro',
        ]);
    }
}
