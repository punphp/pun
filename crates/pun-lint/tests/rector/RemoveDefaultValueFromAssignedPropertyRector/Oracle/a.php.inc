<?php

final class A
{
    private int $a = 1, $b = 2;
    private array $c = [];
    public ?string $d = null;
    private int $e = 0;
    private int $f = 0;

    public function __construct(int $x)
    {
        $this->a = $x;
        if ($x) {
            $this->c = [1];
        } else {
            $this->c = [2];
        }
        $this->d = $this->d . 'x';
        $this->init();
        $fn = function () {
            return 1;
        };
    }

    private function init(): void
    {
        $this->e = 5;
    }
}

class B
{
    private int $a = 1;

    public function __construct()
    {
        foreach ([1] as $i) {
            if ($i) {
                $this->a = 1;
            } else {
                $this->a = 2;
            }
        }
    }
}
-----
<?php

final class A
{
    private int $a, $b = 2;
    private array $c;
    public ?string $d = null;
    private int $e;
    private int $f = 0;

    public function __construct(int $x)
    {
        $this->a = $x;
        if ($x) {
            $this->c = [1];
        } else {
            $this->c = [2];
        }
        $this->d = $this->d . 'x';
        $this->init();
        $fn = function () {
            return 1;
        };
    }

    private function init(): void
    {
        $this->e = 5;
    }
}

class B
{
    private int $a = 1;

    public function __construct()
    {
        foreach ([1] as $i) {
            if ($i) {
                $this->a = 1;
            } else {
                $this->a = 2;
            }
        }
    }
}
