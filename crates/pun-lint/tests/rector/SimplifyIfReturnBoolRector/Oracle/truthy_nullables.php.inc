<?php

namespace Oracle\SimplifyTruthy;

enum Suit
{
    case Hearts;
}

final class Runner
{
    public function enum(?Suit $suit)
    {
        if ($suit) {
            return true;
        }
        return false;
    }

    public function shape(bool $f)
    {
        $value = $f ? ['a' => 1] : null;
        if ($value) {
            return true;
        }
        return false;
    }

    public function yes(bool $f)
    {
        $value = $f ? true : null;
        if ($value) {
            return true;
        }
        return false;
    }

    public function five(bool $f)
    {
        $value = $f ? 5 : null;
        if ($value) {
            return true;
        }
        return false;
    }

    public function half(bool $f)
    {
        $value = $f ? 1.5 : null;
        if ($value) {
            return true;
        }
        return false;
    }

    public function iterable(?iterable $items)
    {
        if ($items) {
            return true;
        }
        return false;
    }

    public function bare(bool $f)
    {
        if ($f) {
            return;
        }
        return true;
    }

    public function nested(int $a, int $b, bool $c)
    {
        if (($a != $b) !== $c) {
            return false;
        }
        return true;
    }

    public function nestedIdentical(int $a, int $b, bool $c)
    {
        if (($a !== $b) != $c) {
            return false;
        }
        return true;
    }
}
-----
<?php

namespace Oracle\SimplifyTruthy;

enum Suit
{
    case Hearts;
}

final class Runner
{
    public function enum(?Suit $suit)
    {
        return $suit !== null;
    }

    public function shape(bool $f)
    {
        $value = $f ? ['a' => 1] : null;
        return $value !== null;
    }

    public function yes(bool $f)
    {
        $value = $f ? true : null;
        return $value !== null;
    }

    public function five(bool $f)
    {
        $value = $f ? 5 : null;
        return $value !== null;
    }

    public function half(bool $f)
    {
        $value = $f ? 1.5 : null;
        return $value !== null;
    }

    public function iterable(?iterable $items)
    {
        return (bool) $items;
    }

    public function bare(bool $f)
    {
        if ($f) {
            return;
        }
        return true;
    }

    public function nested(int $a, int $b, bool $c)
    {
        return ($a != $b) === $c;
    }

    public function nestedIdentical(int $a, int $b, bool $c)
    {
        return ($a !== $b) == $c;
    }
}
