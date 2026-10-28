<?php

namespace App;

use Illuminate\Testing\TestResponse;

// Rector asserts the first status PHPStan lists (`assertNotFound()` here), which checks less than the call did.
function run(TestResponse $response, bool $flag)
{
    $response->assertStatus($flag ? 404 : 403);
}
