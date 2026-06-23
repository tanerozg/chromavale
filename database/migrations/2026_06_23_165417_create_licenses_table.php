<?php

use Illuminate\Database\Migrations\Migration;
use Illuminate\Database\Schema\Blueprint;
use Illuminate\Support\Facades\Schema;

return new class extends Migration
{
    /**
     * Run the migrations.
     */
    public function up(): void
    {
        Schema::create('licenses', function (Blueprint $table) {
            $table->id();
            $table->string('key')->unique();
            $table->string('email');
            $table->string('status')->default('active'); // active | refunded | revoked
            $table->string('stripe_session_id')->nullable()->index();
            $table->string('stripe_payment_intent')->nullable();
            $table->unsignedSmallInteger('max_activations')->default(5);
            $table->json('devices')->nullable(); // array of device ids
            $table->timestamps();
        });
    }

    /**
     * Reverse the migrations.
     */
    public function down(): void
    {
        Schema::dropIfExists('licenses');
    }
};
