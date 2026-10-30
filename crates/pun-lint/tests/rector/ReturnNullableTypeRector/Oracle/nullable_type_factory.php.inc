<?php

final class NullableProbe
{
    public function arrays($x)
    {
        if ($x) {
            return null;
        }
        return [1, 2];
    }

    public function emptyArray($x)
    {
        if ($x) {
            return null;
        }
        return [];
    }

    public function nonEmptyString($x, string $s)
    {
        if ($x) {
            return null;
        }
        return $x ? 'a' . $s : 'b' . $s;
    }

    public function thisOrNew($x)
    {
        if ($x) {
            return null;
        }
        if ($x > 1) {
            return $this;
        }
        return new NullableProbe();
    }

    public function keyed($x, $k)
    {
        if ($x) {
            return null;
        }
        return [$k => 1];
    }

    public function constants($x)
    {
        if ($x) {
            return null;
        }
        if ($x > 1) {
            return 'a';
        }
        return 'b';
    }
}
-----
<?php

final class NullableProbe
{
    public function arrays($x): ?array
    {
        if ($x) {
            return null;
        }
        return [1, 2];
    }

    public function emptyArray($x): ?array
    {
        if ($x) {
            return null;
        }
        return [];
    }

    public function nonEmptyString($x, string $s): ?string
    {
        if ($x) {
            return null;
        }
        return $x ? 'a' . $s : 'b' . $s;
    }

    public function thisOrNew($x): ?\NullableProbe
    {
        if ($x) {
            return null;
        }
        if ($x > 1) {
            return $this;
        }
        return new NullableProbe();
    }

    public function keyed($x, $k): ?array
    {
        if ($x) {
            return null;
        }
        return [$k => 1];
    }

    public function constants($x): ?string
    {
        if ($x) {
            return null;
        }
        if ($x > 1) {
            return 'a';
        }
        return 'b';
    }
}
