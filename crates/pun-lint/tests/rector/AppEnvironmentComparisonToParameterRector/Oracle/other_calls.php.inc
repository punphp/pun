<?php

namespace App;

use Illuminate\Contracts\Foundation\Application;

function run(Application $app)
{
    strlen($app->environment());
    in_array('local', ['local']);
}
