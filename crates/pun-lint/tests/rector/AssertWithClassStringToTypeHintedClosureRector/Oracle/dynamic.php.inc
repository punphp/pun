<?php

namespace App;

use Illuminate\Support\Facades\Bus;

$method = 'assertDispatched';
Bus::$method(SomeJob::class, function ($job) {
    return true;
});
