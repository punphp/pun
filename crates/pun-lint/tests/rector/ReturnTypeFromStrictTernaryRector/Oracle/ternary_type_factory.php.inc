<?php

final class TernaryProbe
{
    public function arrays($x)
    {
        return $x ? [] : [1, 2];
    }

    public function arrayOrNull($x)
    {
        return $x ? null : [1, 2];
    }

    public function thisOrNew($x)
    {
        return $x ? $this : new TernaryProbe();
    }

    public function ints($x)
    {
        return $x ? 1 : 2;
    }

    public function intOrFloat($x)
    {
        return $x ? 1 : 2.5;
    }

    public function stringOrNull($x, string $s)
    {
        return $x ? 'a' . $s : null;
    }

    public function listOrNull($x, string $s)
    {
        return $x ? [$s] : null;
    }
}
-----
<?php

final class TernaryProbe
{
    public function arrays($x)
    {
        return $x ? [] : [1, 2];
    }

    public function arrayOrNull($x)
    {
        return $x ? null : [1, 2];
    }

    public function thisOrNew($x): \TernaryProbe
    {
        return $x ? $this : new TernaryProbe();
    }

    public function ints($x): int
    {
        return $x ? 1 : 2;
    }

    public function intOrFloat($x)
    {
        return $x ? 1 : 2.5;
    }

    public function stringOrNull($x, string $s): ?string
    {
        return $x ? 'a' . $s : null;
    }

    public function listOrNull($x, string $s): ?array
    {
        return $x ? [$s] : null;
    }
}
