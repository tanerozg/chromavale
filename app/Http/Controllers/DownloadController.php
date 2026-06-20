<?php

namespace App\Http\Controllers;

use Illuminate\Http\RedirectResponse;
use Symfony\Component\HttpFoundation\BinaryFileResponse;

class DownloadController extends Controller
{
    /**
     * Serve the installer for the given platform as a direct download.
     *
     * If the binary is present in storage/app/downloads it is streamed
     * straight to the visitor (instant download from our own domain).
     * Otherwise we redirect to the configured fallback URL.
     */
    public function installer(string $platform): BinaryFileResponse|RedirectResponse
    {
        $file = config("download.files.$platform");

        abort_if($file === null, 404);

        $path = storage_path('app/downloads/'.$file);

        if (is_file($path)) {
            return response()->download($path, $file);
        }

        return redirect()->away(config("download.fallback.$platform"));
    }
}
