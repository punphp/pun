<?php

namespace App;

final class Literals
{
    private $flag = false;
    private $ratio;
    private $text;
    private $list;
    private $map;
    private $either;

    public function set(bool $on): void
    {
        $this->flag = true;
        $this->ratio = 1.5;
        $this->text = 'abc';
        $this->list = [1, 2];
        $this->map = ['a' => 1];
        if ($on) {
            $this->either = 1;
        } else {
            $this->either = 2.5;
        }
    }
}
-----
<?php

namespace App;

final class Literals
{
    private bool $flag = false;
    private ?float $ratio = null;
    private ?string $text = null;
    private ?array $list = null;
    private ?array $map = null;
    private int|float|null $either = null;

    public function set(bool $on): void
    {
        $this->flag = true;
        $this->ratio = 1.5;
        $this->text = 'abc';
        $this->list = [1, 2];
        $this->map = ['a' => 1];
        if ($on) {
            $this->either = 1;
        } else {
            $this->either = 2.5;
        }
    }
}
