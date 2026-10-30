<?php

// PHPStan types a variable that holds a `new` as exactly that class, and one that holds a call's or a parameter's
// value as the class or a subclass. Mago's types do not tell them apart, and the answer differs for an interface, so
// Pun leaves every closure that returns such a variable, where Rector changes the last two (the output is the input).

namespace App;

use Illuminate\Contracts\Container\Container;

interface Contract {}
class Open {}

function open(): Open {}

/** @var Container $app */
$app->bind(Contract::class, function () {
    $x = new Open();
    return $x;
});
$app->bind(Contract::class, function () {
    $x = open();
    return $x;
});
$app->bind(Contract::class, function (Open $x) {
    return $x;
});
