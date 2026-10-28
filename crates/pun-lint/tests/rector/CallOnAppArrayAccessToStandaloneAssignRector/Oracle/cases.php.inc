<?php

namespace App;

use Illuminate\Contracts\Foundation\Application;

class Thing
{
    const V = 'validator';

    public function __construct(private Application $app, private \Illuminate\Foundation\Application $full) {}

    public function run(Application $app, $other, ?Application $maybe)
    {
        $a = $app['validator']->make('a');
        // comment
        $b = $app["validator"]->make('b');
        /** @var int $c */
        $c = $this->app['validator']->make([
            'x',
        ]);
        $d = $this->full['validator']->make('d');
        $e = $app[self::V]->make('e');
        $f = $app['other']->make('f');
        $g = $other['validator']->make('g');
        $h = $app['validator']?->make('h');
        $i .= $app['validator']->make('i');
        $app['validator']->make('j');
        $k = $app['validator']->make('k')->fails();
        $l = $maybe['validator']->make('l');
        if (true) {
            $m = $app['validator']->make('m');
        }
        $n = $app['valid' . 'ator']->make('n');
        $o = $app['validator']->make('o'); $p = 1;
        $q = ($app['validator'])->make('q');
    }
}
-----
<?php

namespace App;

use Illuminate\Contracts\Foundation\Application;

class Thing
{
    const V = 'validator';

    public function __construct(private Application $app, private \Illuminate\Foundation\Application $full) {}

    public function run(Application $app, $other, ?Application $maybe)
    {

        /**
         * @var \Illuminate\Validation\Factory $validationFactory
         */
        $validationFactory = $app['validator'];
        $a = $validationFactory->make('a');

        /**
         * @var \Illuminate\Validation\Factory $validationFactory
         */
        $validationFactory = $app["validator"];
        // comment
        $b = $validationFactory->make('b');

        /**
         * @var \Illuminate\Validation\Factory $validationFactory
         */
        $validationFactory = $this->app['validator'];
        /** @var int $c */
        $c = $validationFactory->make([
            'x',
        ]);

        /**
         * @var \Illuminate\Validation\Factory $validationFactory
         */
        $validationFactory = $this->full['validator'];
        $d = $validationFactory->make('d');

        /**
         * @var \Illuminate\Validation\Factory $validationFactory
         */
        $validationFactory = $app[self::V];
        $e = $validationFactory->make('e');
        $f = $app['other']->make('f');
        $g = $other['validator']->make('g');
        $h = $app['validator']?->make('h');
        $i .= $app['validator']->make('i');
        $app['validator']->make('j');
        $k = $app['validator']->make('k')->fails();

        /**
         * @var \Illuminate\Validation\Factory $validationFactory
         */
        $validationFactory = $maybe['validator'];
        $l = $validationFactory->make('l');
        if (true) {

            /**
             * @var \Illuminate\Validation\Factory $validationFactory
             */
            $validationFactory = $app['validator'];
            $m = $validationFactory->make('m');
        }

        /**
         * @var \Illuminate\Validation\Factory $validationFactory
         */
        $validationFactory = $app['valid' . 'ator'];
        $n = $validationFactory->make('n');

        /**
         * @var \Illuminate\Validation\Factory $validationFactory
         */
        $validationFactory = $app['validator'];
        $o = $validationFactory->make('o'); $p = 1;

        /**
         * @var \Illuminate\Validation\Factory $validationFactory
         */
        $validationFactory = $app['validator'];
        $q = ($validationFactory)->make('q');
    }
}
