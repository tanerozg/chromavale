<?php

namespace App\Models;

use Illuminate\Database\Eloquent\Model;

class License extends Model
{
    protected $fillable = [
        'key',
        'email',
        'status',
        'stripe_session_id',
        'stripe_payment_intent',
        'max_activations',
        'devices',
    ];

    protected $casts = [
        'devices' => 'array',
    ];

    protected $attributes = [
        'status' => 'active',
        'max_activations' => 5,
    ];

    /**
     * Generate a unique, human-friendly license key: CV-XXXX-XXXX-XXXX-XXXX.
     */
    public static function generateKey(): string
    {
        $alphabet = 'ABCDEFGHJKLMNPQRSTUVWXYZ23456789'; // no ambiguous chars

        do {
            $groups = [];
            for ($g = 0; $g < 4; $g++) {
                $chunk = '';
                for ($i = 0; $i < 4; $i++) {
                    $chunk .= $alphabet[random_int(0, strlen($alphabet) - 1)];
                }
                $groups[] = $chunk;
            }
            $key = 'CV-'.implode('-', $groups);
        } while (self::where('key', $key)->exists());

        return $key;
    }

    public function isActive(): bool
    {
        return $this->status === 'active';
    }

    /**
     * Register a device against this license. Returns false if the activation
     * limit is reached for a new device.
     */
    public function activateDevice(string $deviceId): bool
    {
        $devices = $this->devices ?? [];

        if (in_array($deviceId, $devices, true)) {
            return true;
        }

        if (count($devices) >= ($this->max_activations ?? 5)) {
            return false;
        }

        $devices[] = $deviceId;
        $this->devices = $devices;
        $this->save();

        return true;
    }
}
