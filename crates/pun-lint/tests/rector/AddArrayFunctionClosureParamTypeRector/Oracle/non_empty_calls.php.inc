<?php

class NonEmptyCalls
{
    public function explodeLiteral(string $s)
    {
        return array_map(static fn ($col) => trim($col), explode(',', $s));
    }

    public function explodeLimit(string $s)
    {
        return array_map(static fn ($col) => trim($col), explode(',', $s, 2));
    }

    public function explodeNegativeLimit(string $s)
    {
        return array_map(static fn ($col) => trim($col), explode(',', $s, -1));
    }

    public function explodeVariableDelimiter(string $d, string $s)
    {
        return array_map(static fn ($col) => trim($col), explode($d, $s));
    }

    public function longRange()
    {
        return array_map(fn ($i) => 'news.'.$i, range(1, 101));
    }

    public function shortRange()
    {
        return array_map(fn ($i) => 'news.'.$i, range(1, 3));
    }

    public function variableRange(int $to)
    {
        return array_map(fn ($i) => 'news.'.$i, range(1, $to));
    }

    public function steppedRange()
    {
        return array_map(fn ($i) => 'news.'.$i, range(1, 10, 2));
    }
}
-----
<?php

class NonEmptyCalls
{
    public function explodeLiteral(string $s)
    {
        return array_map(static fn ($col) => trim($col), explode(',', $s));
    }

    public function explodeLimit(string $s)
    {
        return array_map(static fn ($col) => trim($col), explode(',', $s, 2));
    }

    public function explodeNegativeLimit(string $s)
    {
        return array_map(static fn (string $col) => trim($col), explode(',', $s, -1));
    }

    public function explodeVariableDelimiter(string $d, string $s)
    {
        return array_map(static fn ($col) => trim($col), explode($d, $s));
    }

    public function longRange()
    {
        return array_map(fn ($i) => 'news.'.$i, range(1, 101));
    }

    public function shortRange()
    {
        return array_map(fn (int $i) => 'news.'.$i, range(1, 3));
    }

    public function variableRange(int $to)
    {
        return array_map(fn ($i) => 'news.'.$i, range(1, $to));
    }

    public function steppedRange()
    {
        return array_map(fn (int $i) => 'news.'.$i, range(1, 10, 2));
    }
}
