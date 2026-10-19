<?php

namespace App;

function stop(): never
{
    exit;
}

class Base
{
    public static function halt(): never
    {
        exit;
    }
}

final class NeverCalls extends Base
{
    public static function end(): never
    {
        exit;
    }

    public function dynamicMethod($m, $x)
    {
        $this->$m();
        if ($x) {
            return 1;
        }
        return 'a';
    }

    public function dynamicFunction($f, $x)
    {
        $f();
        if ($x) {
            return 1;
        }
        return 'a';
    }

    public function selfCall($x)
    {
        if ($x) {
            return 1;
        }
        self::end();
    }

    public function staticCall($x)
    {
        if ($x) {
            return 1;
        }
        static::end();
    }

    public function parentCall($x)
    {
        if ($x) {
            return 1;
        }
        parent::halt();
    }

    public function dynamicClass($c, $x)
    {
        $c::end();
        if ($x) {
            return 1;
        }
        return 'a';
    }

    public function qualified($x)
    {
        if ($x) {
            return 1;
        }
        Sub\stop();
        return 'a';
    }
}
-----
<?php

namespace App;

function stop(): never
{
    exit;
}

class Base
{
    public static function halt(): never
    {
        exit;
    }
}

final class NeverCalls extends Base
{
    public static function end(): never
    {
        exit;
    }

    public function dynamicMethod($m, $x): int|string
    {
        $this->$m();
        if ($x) {
            return 1;
        }
        return 'a';
    }

    public function dynamicFunction($f, $x): int|string
    {
        $f();
        if ($x) {
            return 1;
        }
        return 'a';
    }

    public function selfCall($x)
    {
        if ($x) {
            return 1;
        }
        self::end();
    }

    public function staticCall($x)
    {
        if ($x) {
            return 1;
        }
        static::end();
    }

    public function parentCall($x)
    {
        if ($x) {
            return 1;
        }
        parent::halt();
    }

    public function dynamicClass($c, $x): int|string
    {
        $c::end();
        if ($x) {
            return 1;
        }
        return 'a';
    }

    public function qualified($x): int|string
    {
        if ($x) {
            return 1;
        }
        Sub\stop();
        return 'a';
    }
}
