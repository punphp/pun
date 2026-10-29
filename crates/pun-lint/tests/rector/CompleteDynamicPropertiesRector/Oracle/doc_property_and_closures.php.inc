<?php

namespace Pun\Oracle\DynamicProperties;

/**
 * @property int $documented
 */
class DocProperty
{
    public function set()
    {
        $this->documented = 1;
        $this->other = 2;
        $f = function () {
            $this->inClosure = 3;
        };
        $g = fn () => $this->inArrow;
        $this->$name = 4;
    }
}
-----
<?php

namespace Pun\Oracle\DynamicProperties;

/**
 * @property int $documented
 */
class DocProperty
{
    /**
     * @var int
     */
    public $documented;
    /**
     * @var int
     */
    public $other;
    /**
     * @var int
     */
    public $inClosure;
    public $inArrow;
    public function set()
    {
        $this->documented = 1;
        $this->other = 2;
        $f = function () {
            $this->inClosure = 3;
        };
        $g = fn () => $this->inArrow;
        $this->$name = 4;
    }
}
