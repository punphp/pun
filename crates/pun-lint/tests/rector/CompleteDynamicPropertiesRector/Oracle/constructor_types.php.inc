<?php

namespace Pun\Oracle\DynamicProperties;

use Other\Thing;

final class ConstructorTypes
{
    public function __construct(int $i, Thing $t, ?string $s)
    {
        $this->i = $i;
        $this->t = $t;
        $this->s = $s;
        $this->arr = [];
        $this->u = $i ? 1 : 'a';
        $this->m = $this->other();
    }

    // comment
    public function other()
    {
    }
}
-----
<?php

namespace Pun\Oracle\DynamicProperties;

use Other\Thing;

final class ConstructorTypes
{
    private int $i;
    private \Other\Thing $t;
    private ?string $s;
    private array $arr;
    /**
     * @var int|string
     */
    private $u;
    private $m;
    public function __construct(int $i, Thing $t, ?string $s)
    {
        $this->i = $i;
        $this->t = $t;
        $this->s = $s;
        $this->arr = [];
        $this->u = $i ? 1 : 'a';
        $this->m = $this->other();
    }

    // comment
    public function other()
    {
    }
}
