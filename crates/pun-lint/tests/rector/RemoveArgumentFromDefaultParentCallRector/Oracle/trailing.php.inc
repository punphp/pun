<?php

namespace Pun\Oracle\DefaultArg;

class Base
{
    public function run($a, $b = null, $c = false, $d = 'd')
    {
    }

    public function count($a, $b = 0, $c = 1.5, $d = [])
    {
    }
}

class Child extends Base
{
    public function run($a, $b = null, $c = false, $d = 'd')
    {
        parent::run(1, null, false, 'd');
    }

    public function count($a, $b = 0, $c = 1.5, $d = [])
    {
        parent::count($a, 0, 1.5, []);
    }

    public function other($x)
    {
        parent::run($x, null, true);
        parent::run($x, $x);
        parent::run($x, b: null);
        parent::run($x, null, false, 'e');
        parent::count(...);
    }

    private function hidden()
    {
        parent::run(1, null);
    }
}
-----
<?php

namespace Pun\Oracle\DefaultArg;

class Base
{
    public function run($a, $b = null, $c = false, $d = 'd')
    {
    }

    public function count($a, $b = 0, $c = 1.5, $d = [])
    {
    }
}

class Child extends Base
{
    public function run($a, $b = null, $c = false, $d = 'd')
    {
        parent::run(1);
    }

    public function count($a, $b = 0, $c = 1.5, $d = [])
    {
        parent::count($a);
    }

    public function other($x)
    {
        parent::run($x, null, true);
        parent::run($x, $x);
        parent::run($x, b: null);
        parent::run($x, null, false, 'e');
        parent::count(...);
    }

    private function hidden()
    {
        parent::run(1, null);
    }
}
