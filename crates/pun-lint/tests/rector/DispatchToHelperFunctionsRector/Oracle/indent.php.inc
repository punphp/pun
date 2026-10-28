<?php

namespace App;

use Illuminate\Foundation\Bus\Dispatchable;

class Job
{
    use Dispatchable;

    public function __construct($a, $b) {}
}

function run()
{
    Job::dispatch(
        'a',
        [
            'b' => 1,
        ]
    );
    if (true) {
        $x = Job::dispatch([
            'c',
        ], 'd');
    }
}
-----
<?php

namespace App;

use Illuminate\Foundation\Bus\Dispatchable;

class Job
{
    use Dispatchable;

    public function __construct($a, $b) {}
}

function run()
{
    dispatch(new \App\Job('a', [
        'b' => 1,
    ]));
    if (true) {
        $x = dispatch(new \App\Job([
            'c',
        ], 'd'));
    }
}
