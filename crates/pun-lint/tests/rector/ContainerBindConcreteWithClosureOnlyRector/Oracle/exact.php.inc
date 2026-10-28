<?php

namespace App;

use Illuminate\Contracts\Container\Container;

interface Contract {}
interface Other2 {}
class Open {}
class Sub extends Open {}
class Impl implements Contract {}
final class Other {}

function make(): Other2 {}
function open(): Open {}

/** @var Container $app */
$app->bind(Contract::class, function () {
    return new Open();
});
$app->bind(Contract::class, function () {
    return rand() ? new Open() : new Other();
});
$app->bind(Contract::class, function () {
    if (rand()) {
        return new Open();
    }
    return new Other();
});
$app->bind(Contract::class, function () {
    return rand() ? new Open() : new Open();
});
$app->bind(Sub::class, function () {
    return new Open();
});
$app->bind(Open::class, function () {
    return new Sub();
});
$app->bind(Contract::class, function () {
    return make();
});
$app->bind(Contract::class, function () {
    return open();
});
$app->bind(Open::class, function () {
    return make();
});
$app->bind(Impl::class, function () {
    return make();
});
$app->bind(Contract::class, function () {
    return open() ?: new Open();
});
$app->bind(Contract::class, function () {
    return rand() ? open() : open();
});
-----
<?php

namespace App;

use Illuminate\Contracts\Container\Container;

interface Contract {}
interface Other2 {}
class Open {}
class Sub extends Open {}
class Impl implements Contract {}
final class Other {}

function make(): Other2 {}
function open(): Open {}

/** @var Container $app */
$app->bind(Contract::class, function () {
    return new Open();
});
$app->bind(Contract::class, function () {
    return rand() ? new Open() : new Other();
});
$app->bind(Contract::class, function () {
    if (rand()) {
        return new Open();
    }
    return new Other();
});
$app->bind(Contract::class, function () {
    return rand() ? new Open() : new Open();
});
$app->bind(Sub::class, function () {
    return new Open();
});
$app->bind(function (): \App\Open {
    return new Sub();
});
$app->bind(function (): \App\Contract {
    return make();
});
$app->bind(function (): \App\Contract {
    return open();
});
$app->bind(function (): \App\Open {
    return make();
});
$app->bind(function (): \App\Impl {
    return make();
});
$app->bind(function (): \App\Contract {
    return open() ?: new Open();
});
$app->bind(function (): \App\Contract {
    return rand() ? open() : open();
});
