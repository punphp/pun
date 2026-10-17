<?php

declare(strict_types=1);

namespace App\Providers;

use Carbon\CarbonImmutable;
use Illuminate\Support\Facades\Date;
use Illuminate\Support\ServiceProvider;

/** The line of Pinkary's AppServiceProvider::boot() that sets the date class. */
final class AppServiceProvider extends ServiceProvider
{
    private const DATE_CLASS = CarbonImmutable::class;

    public function boot(): void
    {
        Date::use(self::DATE_CLASS);
        $date::use(CarbonImmutable::class);
        Date::use(CarbonImmutable::class);
        Date::now();
        $this->app->bind('x', 'y');
    }
}
