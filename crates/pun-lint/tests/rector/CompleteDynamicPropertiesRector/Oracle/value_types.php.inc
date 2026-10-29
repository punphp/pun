<?php

namespace Pun\Oracle\DynamicProperties;

use Other\Thing;

class Known
{
}

class ValueTypes
{
    public function set()
    {
        $this->a = 1;
        $this->b = 'x';
        $this->c = new Thing();
        $this->d = [1, 2];
        $this->e = null;
        $this->f = Thing::class;
        $this->g = 1.5;
        $this->h = true;
        $this->i = new Known();
        $this->j = new \DateTime();
        $this->k = Known::class;
        $this->l = [new Known()];
        $this->m = ['a' => 1];
    }
}
-----
<?php

namespace Pun\Oracle\DynamicProperties;

use Other\Thing;

class Known
{
}

class ValueTypes
{
    /**
     * @var int
     */
    public $a;
    /**
     * @var string
     */
    public $b;
    /**
     * @var Other\Thing
     */
    public $c;
    /**
     * @var int[]
     */
    public $d;
    /**
     * @var null
     */
    public $e;
    /**
     * @var string
     */
    public $f;
    /**
     * @var float
     */
    public $g;
    /**
     * @var bool
     */
    public $h;
    /**
     * @var \Pun\Oracle\DynamicProperties\Known
     */
    public $i;
    /**
     * @var \DateTime
     */
    public $j;
    /**
     * @var class-string<\Pun\Oracle\DynamicProperties\Known>
     */
    public $k;
    /**
     * @var \Pun\Oracle\DynamicProperties\Known[]
     */
    public $l;
    /**
     * @var array<string, int>
     */
    public $m;
    public function set()
    {
        $this->a = 1;
        $this->b = 'x';
        $this->c = new Thing();
        $this->d = [1, 2];
        $this->e = null;
        $this->f = Thing::class;
        $this->g = 1.5;
        $this->h = true;
        $this->i = new Known();
        $this->j = new \DateTime();
        $this->k = Known::class;
        $this->l = [new Known()];
        $this->m = ['a' => 1];
    }
}
