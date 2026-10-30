<?php

namespace Pun\Oracle\DynamicProperties;

class ReadsAndDims
{
    public function get()
    {
        return $this->r;
    }

    public function set()
    {
        $this->r = 5;
    }

    public function dim(int $i)
    {
        $this->list[] = $i;
        $this->map[$i] = 'v';
    }
}
-----
<?php

namespace Pun\Oracle\DynamicProperties;

class ReadsAndDims
{
    public $r;
    /**
     * @var int[]
     */
    public $list;
    /**
     * @var string[]
     */
    public $map;
    public function get()
    {
        return $this->r;
    }

    public function set()
    {
        $this->r = 5;
    }

    public function dim(int $i)
    {
        $this->list[] = $i;
        $this->map[$i] = 'v';
    }
}
