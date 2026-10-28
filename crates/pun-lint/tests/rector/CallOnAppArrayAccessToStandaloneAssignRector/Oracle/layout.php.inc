<?php

namespace App;

use Illuminate\Contracts\Foundation\Application;

function run(Application $app)
{
    $p = 1; $r = $app['validator']->make('r');
    $s = 2; // trailing
    $t = $app['validator']->make(
        't'
    );
}
-----
<?php

namespace App;

use Illuminate\Contracts\Foundation\Application;

function run(Application $app)
{
    $p = 1;

    /**
     * @var \Illuminate\Validation\Factory $validationFactory
     */
    $validationFactory = $app['validator']; $r = $validationFactory->make('r');
    $s = 2;

    /**
     * @var \Illuminate\Validation\Factory $validationFactory
     */
    $validationFactory = $app['validator']; // trailing
    $t = $validationFactory->make(
        't'
    );
}
