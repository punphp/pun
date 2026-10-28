<?php

namespace App;

use Illuminate\Contracts\Container\Container;

interface Contract {}

function run(Container $app, callable $factory)
{
    $app->bind(Contract::class, function () use ($factory) {
        return $factory;
    });
}
-----
<?php

namespace App;

use Illuminate\Contracts\Container\Container;

interface Contract {}

function run(Container $app, callable $factory)
{
    $app->bind(function () use ($factory): \App\Contract {
        return $factory;
    });
}
