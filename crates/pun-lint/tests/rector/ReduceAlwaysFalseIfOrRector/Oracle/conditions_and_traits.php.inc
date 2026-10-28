<?php

namespace Oracle\AlwaysFalse;

trait Shared
{
    public function inTrait(int $n, \stdClass $o)
    {
        if (!is_int($n) || $n > 5) {
            return 1;
        }
        if (!$o instanceof \stdClass || $n > 5) {
            return 2;
        }
        return 0;
    }
}

final class Runner
{
    private int $count = 0;

    public function run(int $n, array $items, $untyped)
    {
        if ((!is_int($n)) || ($n > 1 && $n < 9)) {
            echo 1;
        }
        if (!is_int($n) || $n > 1 || $n < -1) {
            echo 2;
        }
        if (!is_int($this->count) || $n > 1) {
            echo 3;
        }
        if (!is_array($items) || $items['a'] > 1) {
            echo 4;
        }
        if (!is_int($n) or $n > 1) {
            echo 5;
        }
        if (!is_int($n) || $n > 1) {
            echo 6;
        } elseif ($n < 0) {
            echo 7;
        }
        if (!is_array($items) || $untyped) {
            echo 8;
        }
        if (false || $n > 2) {
            echo 9;
        }
        if (!$this->make() || $n > 2) {
            echo 10;
        }
        if (!$this->untypedMake() || $n > 2) {
            echo 11;
        }
        if (!is_int($n + strlen('x')) || $n > 2) {
            echo 12;
        }
    }

    private function make(): \stdClass
    {
        return new \stdClass();
    }

    private function untypedMake()
    {
        return new \stdClass();
    }
}
-----
<?php

namespace Oracle\AlwaysFalse;

trait Shared
{
    public function inTrait(int $n, \stdClass $o)
    {
        if ($n > 5) {
            return 1;
        }
        if ($n > 5) {
            return 2;
        }
        return 0;
    }
}

final class Runner
{
    private int $count = 0;

    public function run(int $n, array $items, $untyped)
    {
        if ($n > 1 && $n < 9) {
            echo 1;
        }
        if (!is_int($n) || $n > 1 || $n < -1) {
            echo 2;
        }
        if (!is_int($this->count) || $n > 1) {
            echo 3;
        }
        if ($items['a'] > 1) {
            echo 4;
        }
        if (!is_int($n) or $n > 1) {
            echo 5;
        }
        if ($n > 1) {
            echo 6;
        } elseif ($n < 0) {
            echo 7;
        }
        if ($untyped) {
            echo 8;
        }
        if ($n > 2) {
            echo 9;
        }
        if ($n > 2) {
            echo 10;
        }
        if (!$this->untypedMake() || $n > 2) {
            echo 11;
        }
        if ($n > 2) {
            echo 12;
        }
    }

    private function make(): \stdClass
    {
        return new \stdClass();
    }

    private function untypedMake()
    {
        return new \stdClass();
    }
}
