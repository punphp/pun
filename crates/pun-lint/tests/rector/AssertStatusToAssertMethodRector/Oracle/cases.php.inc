<?php

namespace App;

use Illuminate\Testing\TestResponse;

class Codes
{
    public const OK = 200;
}

function run(TestResponse $response, ?TestResponse $maybe, bool $flag, int $code)
{
    $response->ASSERTSTATUS(204);
    $response->assertStatus(status: 401);
    $response->assertStatus(Codes::OK);
    $maybe->assertStatus(405);
    $response->assertStatus(410)->assertStatus(422);
    $response->assertStatus(500 /* comment */);
    $response->assertStatus(503);
    $response->assertStatus(201);
    $response->assertStatus('200');
    $response->assertStatus($code);
    $response->assertStatus(200.0);
    $response?->assertStatus(200);
    $response
        ->assertStatus(200);
    $response->assertHeader('a', 'b')
        ->assertStatus(404)
        ->assertHeader('c', 'd');
    $response->
        assertStatus(403);
    ($response)
        ->assertStatus(410);
    (clone $response)
        ->assertStatus(200);
    $response->assertStatus(200)
        ->assertStatus(404);
}
-----
<?php

namespace App;

use Illuminate\Testing\TestResponse;

class Codes
{
    public const OK = 200;
}

function run(TestResponse $response, ?TestResponse $maybe, bool $flag, int $code)
{
    $response->assertNoContent();
    $response->assertUnauthorized();
    $response->assertOk();
    $maybe->assertMethodNotAllowed();
    $response->assertGone()->assertStatus(422);
    $response->assertInternalServerError();
    $response->assertServiceUnavailable();
    $response->assertCreated();
    $response->assertStatus('200');
    $response->assertStatus($code);
    $response->assertStatus(200.0);
    $response?->assertStatus(200);
    $response->assertOk();
    $response->assertHeader('a', 'b')
        ->assertStatus(404)
        ->assertHeader('c', 'd');
    $response->assertForbidden();
    $response->assertGone();
    (clone $response)->assertOk();
    $response->assertOk()
        ->assertStatus(404);
}
