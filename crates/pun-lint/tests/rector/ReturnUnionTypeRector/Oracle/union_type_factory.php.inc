<?php

final class Probe
{
    public function emptyOrList(string $a, string $b, $x)
    {
        if ($x) {
            return [];
        }
        return [$a, $b];
    }

    public function constList()
    {
        return [1, 2];
    }

    public function oneItem(string $a)
    {
        return [$a];
    }

    public function nonEmpty($k)
    {
        if ($k) {
            return null;
        }
        return [$k => 1];
    }

    public function strings(string $s, $x)
    {
        if ($x) {
            return 'a';
        }
        return 'x' . $s;
    }

    public function thisOrNew($x)
    {
        if ($x) {
            return new Probe();
        }
        return $this;
    }

    public function thisOrNull($x)
    {
        if ($x) {
            return null;
        }
        return $this;
    }

    public function ints($x)
    {
        if ($x) {
            return 1;
        }
        return 2;
    }

    public function keyKinds($x)
    {
        if ($x) {
            return [1];
        }
        return ['a' => 1];
    }

    public function intsNull($x, $y)
    {
        if ($x) {
            return 1;
        }
        if ($y) {
            return null;
        }
        return 2;
    }

    public function mixedItem($x)
    {
        return [$x, 1];
    }

    public function sameArrays($x)
    {
        if ($x) {
            return [1];
        }
        return [1];
    }

    public function listOrNull(array $a, $x)
    {
        if ($x) {
            return null;
        }
        return array_values($a);
    }

    public function stringOrNonEmpty(string $s, $x)
    {
        if ($x) {
            return $s;
        }
        return 'x' . $s;
    }

    public function boolAndTrue(bool $b, $x)
    {
        if ($x) {
            return true;
        }
        return $b;
    }
}
-----
<?php

final class Probe
{
    public function emptyOrList(string $a, string $b, $x): array
    {
        if ($x) {
            return [];
        }
        return [$a, $b];
    }

    public function constList(): array
    {
        return [1, 2];
    }

    public function oneItem(string $a)
    {
        return [$a];
    }

    public function nonEmpty($k)
    {
        if ($k) {
            return null;
        }
        return [$k => 1];
    }

    public function strings(string $s, $x)
    {
        if ($x) {
            return 'a';
        }
        return 'x' . $s;
    }

    public function thisOrNew($x)
    {
        if ($x) {
            return new Probe();
        }
        return $this;
    }

    public function thisOrNull($x)
    {
        if ($x) {
            return null;
        }
        return $this;
    }

    public function ints($x)
    {
        if ($x) {
            return 1;
        }
        return 2;
    }

    public function keyKinds($x): array
    {
        if ($x) {
            return [1];
        }
        return ['a' => 1];
    }

    public function intsNull($x, $y)
    {
        if ($x) {
            return 1;
        }
        if ($y) {
            return null;
        }
        return 2;
    }

    public function mixedItem($x)
    {
        return [$x, 1];
    }

    public function sameArrays($x)
    {
        if ($x) {
            return [1];
        }
        return [1];
    }

    public function listOrNull(array $a, $x)
    {
        if ($x) {
            return null;
        }
        return array_values($a);
    }

    public function stringOrNonEmpty(string $s, $x)
    {
        if ($x) {
            return $s;
        }
        return 'x' . $s;
    }

    public function boolAndTrue(bool $b, $x)
    {
        if ($x) {
            return true;
        }
        return $b;
    }
}
