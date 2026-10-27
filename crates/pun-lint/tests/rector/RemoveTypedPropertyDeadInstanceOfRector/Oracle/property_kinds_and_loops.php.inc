<?php

namespace App;

use stdClass;

class Base
{
    protected ?stdClass $fromParent = null;
}

class Child extends \ArrayObject
{
}

final class Holder extends Base
{
    private stdClass $typed;
    private ?stdClass $nullable;
    /** @var stdClass */
    private $documented;
    private $inferred;
    private $inferredParam;
    private $inferredLast;
    public $public;
    private Child $child;
    private \ArrayObject $object;

    public function __construct(stdClass $param, private stdClass $promoted)
    {
        $this->typed = new stdClass();
        $this->nullable = null;
        $this->documented = $param;
        $this->inferred = new stdClass();
        $this->inferredParam = $param;
        $this->inferredLast = new stdClass();
        $this->inferredLast = new \ArrayObject();
        $this->public = new stdClass();
        $this->fromParent = new stdClass();
        $this->child = new Child();
        $this->object = new \ArrayObject();
    }

    public function first()
    {
        echo 0;
        if ($this->typed instanceof stdClass) {
            echo 1;
        }
        if (!$this->promoted instanceof stdClass) {
            return;
        }
        if ($this->nullable instanceof stdClass) {
            echo 2;
        }
        if ($this->documented instanceof stdClass) {
            echo 3;
        }
        if ($this->inferred instanceof stdClass) {
            echo 4;
        }
        if ($this->inferredParam instanceof stdClass) {
            echo 5;
        }
        if ($this->inferredLast instanceof stdClass) {
            echo 6;
        }
        if ($this->public instanceof stdClass) {
            echo 7;
        }
        if ($this->fromParent instanceof stdClass) {
            echo 8;
        }
        if ($this->child instanceof \ArrayObject) {
            echo 9;
        }
        if ($this->object instanceof Child) {
            echo 10;
        }
        if ($this->typed instanceof stdClass && true) {
            echo 11;
        }
        if ($this->typed instanceof stdClass) {
            foreach ([1] as $i) {
                echo $i;
            }
        }
    }

    public function second()
    {
        if ($this->typed instanceof stdClass) {
            echo 12;
        }
        while (false) {
        }
        if ($this->typed instanceof stdClass) {
            echo 13;
        }
    }

    public function third()
    {
        if ($this->typed instanceof stdClass) {
            echo 14;
        }
    }
}
-----
<?php

namespace App;

use stdClass;

class Base
{
    protected ?stdClass $fromParent = null;
}

class Child extends \ArrayObject
{
}

final class Holder extends Base
{
    private stdClass $typed;
    private ?stdClass $nullable;
    /** @var stdClass */
    private $documented;
    private $inferred;
    private $inferredParam;
    private $inferredLast;
    public $public;
    private Child $child;
    private \ArrayObject $object;

    public function __construct(stdClass $param, private stdClass $promoted)
    {
        $this->typed = new stdClass();
        $this->nullable = null;
        $this->documented = $param;
        $this->inferred = new stdClass();
        $this->inferredParam = $param;
        $this->inferredLast = new stdClass();
        $this->inferredLast = new \ArrayObject();
        $this->public = new stdClass();
        $this->fromParent = new stdClass();
        $this->child = new Child();
        $this->object = new \ArrayObject();
    }

    public function first()
    {
        echo 0;
        echo 1;
        if ($this->nullable instanceof stdClass) {
            echo 2;
        }
        echo 3;
        echo 4;
        echo 5;
        if ($this->inferredLast instanceof stdClass) {
            echo 6;
        }
        if ($this->public instanceof stdClass) {
            echo 7;
        }
        if ($this->fromParent instanceof stdClass) {
            echo 8;
        }
        echo 9;
        if ($this->object instanceof Child) {
            echo 10;
        }
        if ($this->typed instanceof stdClass && true) {
            echo 11;
        }
        foreach ([1] as $i) {
            echo $i;
        }
    }

    public function second()
    {
        echo 12;
        while (false) {
        }
        if ($this->typed instanceof stdClass) {
            echo 13;
        }
    }

    public function third()
    {
        if ($this->typed instanceof stdClass) {
            echo 14;
        }
    }
}
