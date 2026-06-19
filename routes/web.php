<?php

use App\Http\Controllers\WaitlistController;
use Illuminate\Support\Facades\Route;

Route::inertia('/', 'Welcome')->name('home');
Route::inertia('try', 'Try')->name('try');
Route::inertia('download', 'Download')->name('download');
Route::post('waitlist', [WaitlistController::class, 'store'])->name('waitlist.store');

Route::middleware(['auth', 'verified'])->group(function () {
    Route::inertia('dashboard', 'Dashboard')->name('dashboard');
    Route::get('admin/waitlist', [WaitlistController::class, 'index'])->name('waitlist.index');
});

require __DIR__.'/settings.php';
