<?php

namespace App;

use Illuminate\Support\Facades\Bus;

// Rector stops on this file: a `string` is no class string, but its `getClassStringObjectType()` is `object`, whose
// `getClassName()` does not exist. Pun leaves the call.
function run(string $plain)
{
    Bus::assertDispatched($plain, fn ($job) => true);
}
