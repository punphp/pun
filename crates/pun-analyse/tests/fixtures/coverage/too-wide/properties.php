<?php

namespace CoverageTooWideProperties;

trait Tr
{
    private ?int $t = 1;
}

class Other
{
    public ?int $o = null;
}

final class Props
{
    use Tr;

    private ?int $a = 1;
    private ?int $b = null;
    private int|string $c = 1;
    private int|string $d = 1;
    private int|string $e = 1;
    private ?array $f = [];
    private int|string $g = 1;
    private ?int $h = 1;
    private ?int $i = 1;
    /** @var int|string */
    private $j;
    private static ?int $k = 1;
    private ?int $l;
    public ?int $m = 1;
    private ?int $hooked = 1 {
        get => $this->hooked;
    }

    public function __construct(private ?int $promoted = 1)
    {
        $this->l = 1;
    }

    public function run(Other $o, array $list): void
    {
        $this->b = 1;
        $this->c .= 'x';
        $x = 1;
        $this->d = &$x;
        $this->f[] = 1;
        $this->g++;
        foreach ($list as $this->h) {
        }
        $this->take($this->i);
        $this->j = 1;
        self::$k = 2;
        $o->o = 1;
    }

    public function readF(): ?array
    {
        return $this->f;
    }

    private function take(?int $v): void
    {
    }
}

final class Dynamic
{
    private ?int $a = 1;

    public function run(string $n): void
    {
        $this->{$n} = 1;
    }
}

$anon = new class {
    private ?int $a = 1;
};
