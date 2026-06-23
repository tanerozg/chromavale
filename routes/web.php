<?php

use App\Http\Controllers\CheckoutController;
use App\Http\Controllers\DownloadController;
use App\Http\Controllers\LicenseController;
use App\Http\Controllers\WaitlistController;
use Illuminate\Support\Facades\Route;

Route::inertia('/', 'Welcome')->name('home');
Route::inertia('try', 'Try')->name('try');
Route::inertia('calibrate', 'Calibrate')->name('calibrate');
Route::inertia('download', 'Download')->name('download');
Route::get('download/{platform}', [DownloadController::class, 'installer'])
    ->whereIn('platform', ['windows', 'mac'])
    ->name('download.installer');
Route::post('waitlist', [WaitlistController::class, 'store'])->name('waitlist.store');

// Pro purchase (Stripe) and licensing.
Route::get('checkout/pro', [CheckoutController::class, 'pro'])->name('checkout.pro');
Route::inertia('pro/thanks', 'PurchaseSuccess')->name('pro.thanks');
Route::post('stripe/webhook', [CheckoutController::class, 'webhook'])->name('stripe.webhook');
Route::post('api/license/activate', [LicenseController::class, 'activate'])->name('license.activate');

Route::middleware(['auth', 'verified'])->group(function () {
    Route::inertia('dashboard', 'Dashboard')->name('dashboard');
    Route::get('admin/waitlist', [WaitlistController::class, 'index'])->name('waitlist.index');
});

require __DIR__.'/settings.php';
