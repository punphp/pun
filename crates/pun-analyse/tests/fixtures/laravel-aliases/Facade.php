<?php

// Reduced from laravel/framework v13.23.0, Support/Facades/Facade.php::defaultAliases().
// Keep both imported targets (Arr, Model) and targets relative to the facade namespace.
namespace Illuminate\Support\Facades;

use Illuminate\Database\Eloquent\Model;
use Illuminate\Support\Arr;
use Illuminate\Support\Collection;

abstract class Facade
{
    public static function defaultAliases()
    {
        return new Collection([
            'Arr' => Arr::class,
            'Blade' => Blade::class,
            'Eloquent' => Model::class,
        ]);
    }
}
