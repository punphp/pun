<?php

class TBase
{
}

final class TChild extends TBase
{
}

enum TSuit
{
    case Hearts;
    case Spades;
}

final class TernaryProbe2
{
    public function childBase($x)
    {
        return $x ? new TChild() : new TBase();
    }

    public function cases($x)
    {
        return $x ? TSuit::Hearts : TSuit::Spades;
    }

    public function thisNull($x)
    {
        return $x ? $this : null;
    }

    public function stringsNull($x)
    {
        return $x ? 'a' : ($x > 1 ? 'b' : null);
    }

    public function withOtherReturn($x)
    {
        if ($x > 5) {
            return null;
        }
        return $x ? 1 : 2;
    }

    public function listItems($x, string $s)
    {
        return $x ? [$s] : [];
    }

    public function stringNonEmpty($x, string $s)
    {
        return $x ? $s : 'a' . $s;
    }

    public function boolOrNull($x)
    {
        return $x ? true : null;
    }

    public function trueOrFalse($x)
    {
        return $x ? true : false;
    }
}
-----
<?php

class TBase
{
}

final class TChild extends TBase
{
}

enum TSuit
{
    case Hearts;
    case Spades;
}

final class TernaryProbe2
{
    public function childBase($x)
    {
        return $x ? new TChild() : new TBase();
    }

    public function cases($x): \TSuit
    {
        return $x ? TSuit::Hearts : TSuit::Spades;
    }

    public function thisNull($x): ?self
    {
        return $x ? $this : null;
    }

    public function stringsNull($x): ?string
    {
        return $x ? 'a' : ($x > 1 ? 'b' : null);
    }

    public function withOtherReturn($x)
    {
        if ($x > 5) {
            return null;
        }
        return $x ? 1 : 2;
    }

    public function listItems($x, string $s)
    {
        return $x ? [$s] : [];
    }

    public function stringNonEmpty($x, string $s): string
    {
        return $x ? $s : 'a' . $s;
    }

    public function boolOrNull($x): ?bool
    {
        return $x ? true : null;
    }

    public function trueOrFalse($x): bool
    {
        return $x ? true : false;
    }
}
