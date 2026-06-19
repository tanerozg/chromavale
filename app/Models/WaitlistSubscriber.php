<?php

namespace App\Models;

use Illuminate\Database\Eloquent\Model;

class WaitlistSubscriber extends Model
{
    protected $fillable = [
        'email',
        'platform',
    ];
}
