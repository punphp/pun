<?php

namespace Oracle\ExplicitNull;

use Foo\Bar;

final class Runner
{
    /**
     * @return void|\Foo\Baz|Bar
     */
    public function classes(int $n)
    {
        if ($n > 1) {
            return new \stdClass();
        }
    }

    /**
     * @return int|void|null
     */
    public function nulls(int $n)
    {
        if ($n > 1) {
            return 1;
        }
    }

    /**
     * @return mixed|void
     */
    public function mixed(int $n)
    {
        if ($n > 1) {
            return 1;
        }
    }

    /** @return string|void */
    public function inline(int $n)
    {
        if ($n > 1) {
            return 'x';
        }
        // trailing
    }

    public function bare(int $n)
    {
        if ($n > 1) {
            return ;
        }
        return 5;
    }

    public function mixedValue($n)
    {
        if ($n > 1) {
            return $n;
        }
    }

    public function closures(int $n)
    {
        $f = function () {
            return;
        };
        if ($n) {
            return 1;
        }
    }
}
-----
<?php

namespace Oracle\ExplicitNull;

use Foo\Bar;

final class Runner
{
    /**
     * @return null|Foo\Baz|Bar
     */
    public function classes(int $n)
    {
        if ($n > 1) {
            return new \stdClass();
        }
        return null;
    }

    /**
     * @return int|null
     */
    public function nulls(int $n)
    {
        if ($n > 1) {
            return 1;
        }
        return null;
    }

    /**
     * @return mixed|void
     */
    public function mixed(int $n)
    {
        if ($n > 1) {
            return 1;
        }
        return null;
    }

    /** @return string|null */
    public function inline(int $n)
    {
        if ($n > 1) {
            return 'x';
        }
        // trailing
        return null;
    }

    public function bare(int $n)
    {
        if ($n > 1) {
            return null ;
        }
        return 5;
    }

    public function mixedValue($n)
    {
        if ($n > 1) {
            return $n;
        }
    }

    public function closures(int $n)
    {
        $f = function () {
            return;
        };
        if ($n) {
            return 1;
        }
        return null;
    }
}
