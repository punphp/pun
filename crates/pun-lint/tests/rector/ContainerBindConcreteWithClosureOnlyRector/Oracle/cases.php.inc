<?php

namespace App;

use Illuminate\Contracts\Container\Container;

interface Contract {}
class Impl implements Contract {}
final class Other {}
class Open {}
final class FinalImpl implements Contract {}

/** @var Container $app */
$app->bind(Contract::class, function () {
    return new Impl();
});
$app->singleton(Contract::class, function ($app) use ($x) {
    return new Impl();
});
$app->bindIf(Contract::class, static function (): Impl {
    return new Impl();
});
$app->SINGLETONIF(Contract::class, function () {
    return new Other();
});
$app->bind(Open::class, function () {
    return new Other();
});
$app->bind(Contract::class, function () {
    return new FinalImpl();
}, true);
$app->bind(Contract::class, function () {
    if (rand()) {
        return new Impl();
    }
});
$app->bind(Contract::class, function () {
    echo 1;
});
$app->bind(Contract::class, function () {
    return;
});
$app->bind(Contract::class, function () {
    return 1;
});
$app->bind(Contract::class, function () use ($app) {
    return $app->make(Impl::class);
});
$app->bind(Contract::class, function () {
    return rand() ? new Other() : null;
});
$app->bind(Contract::class, function () {
    return rand() ? new Other() : new Open();
});
$app->bind(Contract::class, fn () => new Impl());
$app->bind('App\Contract', function () {
    return new Impl();
});
$app->bind('App\Missing', function () {
    return new Impl();
});
$app->bind(Missing::class, function () {
    return new Impl();
});
$app->bind(Contract::class);
$app->bind(Contract::class, Impl::class);
$abstract = Contract::class;
$app->bind($abstract, function () {
    return new Impl();
});
$app->bind(abstract: Contract::class, concrete: function () {
    return new Impl();
});
$app->bind((Contract::class), (function () {
    return new Impl();
}));
$app->bind(Contract::class, function () {
    return function () {};
});
$app->bind(\Closure::class, function () {
    return function () {};
});
$app?->bind(Contract::class, function () {
    return new Impl();
});
/** @var \Illuminate\Container\Container $container */
$container->bind(Contract::class, function () {
    return new Impl();
});
$app->bind(Contract::class, function () {
    yield new Impl();
});
$app->bind(Other::class, function () {
    return new Open();
});
$app->bind(Suit::class, function () {
    return new Open();
});
$app->bind(Open::class, function () {
    return new class {};
});

enum Suit
{
    case A;
}
-----
<?php

namespace App;

use Illuminate\Contracts\Container\Container;

interface Contract {}
class Impl implements Contract {}
final class Other {}
class Open {}
final class FinalImpl implements Contract {}

/** @var Container $app */
$app->bind(function (): \App\Contract {
    return new Impl();
});
$app->singleton(function ($app) use ($x): \App\Contract {
    return new Impl();
});
$app->bindIf(static function (): \App\Contract {
    return new Impl();
});
$app->SINGLETONIF(Contract::class, function () {
    return new Other();
});
$app->bind(Open::class, function () {
    return new Other();
});
$app->bind(function (): \App\Contract {
    return new FinalImpl();
}, true);
$app->bind(function (): \App\Contract {
    if (rand()) {
        return new Impl();
    }
});
$app->bind(function (): \App\Contract {
    echo 1;
});
$app->bind(function (): \App\Contract {
    return;
});
$app->bind(Contract::class, function () {
    return 1;
});
$app->bind(function () use ($app): \App\Contract {
    return $app->make(Impl::class);
});
$app->bind(Contract::class, function () {
    return rand() ? new Other() : null;
});
$app->bind(Contract::class, function () {
    return rand() ? new Other() : new Open();
});
$app->bind(Contract::class, fn () => new Impl());
$app->bind(function (): \App\Contract {
    return new Impl();
});
$app->bind('App\Missing', function () {
    return new Impl();
});
$app->bind(function (): \App\Missing {
    return new Impl();
});
$app->bind(Contract::class);
$app->bind(Contract::class, Impl::class);
$abstract = Contract::class;
$app->bind($abstract, function () {
    return new Impl();
});
$app->bind(concrete: function (): \App\Contract {
    return new Impl();
});
$app->bind((function (): \App\Contract {
    return new Impl();
}));
$app->bind(Contract::class, function () {
    return function () {};
});
$app->bind(function (): \Closure {
    return function () {};
});
$app?->bind(Contract::class, function () {
    return new Impl();
});
/** @var \Illuminate\Container\Container $container */
$container->bind(function (): \App\Contract {
    return new Impl();
});
$app->bind(function (): \App\Contract {
    yield new Impl();
});
$app->bind(Other::class, function () {
    return new Open();
});
$app->bind(Suit::class, function () {
    return new Open();
});
$app->bind(function (): \App\Open {
    return new class {};
});

enum Suit
{
    case A;
}
