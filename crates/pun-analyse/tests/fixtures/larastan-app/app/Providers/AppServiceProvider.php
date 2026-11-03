<?php

namespace App\Providers;

use Carbon\CarbonImmutable;
use Illuminate\Support\Facades\Date;

final class AppServiceProvider
{
    public function boot(): void
    {
        Date::use(CarbonImmutable::class);
    }
}
