<?php

namespace App\Conditions;

use stdClass;

function outside(stdClass $value)
{
    if ($value instanceof stdClass) {
        echo 0;
    }
}

class Other
{
}

final class Holder
{
    private stdClass $typed;
    private stdClass|Other $union;
    private $inferred;
    private $text = '';
    public stdClass $open;

    public function __construct(stdClass $param)
    {
        echo 'start';
        strlen('x');
        $local = 1;
        $this->text .= 'more';
        $this->typed = $param;
        $this->union = $param;
        $this->inferred = new stdClass();
        $this->open = $param;
    }

    public function conditions(bool $flag, stdClass $value, self $other, string $class, string $name)
    {
        if ($this->typed instanceof stdClass) {
            echo 1;
        } else {
            echo 2;
        }
        if (!$flag) {
            echo 3;
        }
        if ($flag) {
            echo 4;
        }
        if ($value instanceof stdClass) {
            echo 5;
        }
        if ($other->typed instanceof stdClass) {
            echo 6;
        }
        if ($this->typed instanceof $class) {
            echo 7;
        }
        if ($this->{$name} instanceof stdClass) {
            echo 8;
        }
        if ($this->union instanceof stdClass) {
            echo 9;
        }
        if ($this->inferred instanceof stdClass) {
            echo 10;
        }
        if ($this->open instanceof stdClass) {
            echo 11;
        }
    }

    public function anonymous()
    {
        return new class (new stdClass()) {
            public function __construct(private stdClass $inner)
            {
            }

            public function run()
            {
                if ($this->inner instanceof stdClass) {
                    echo 12;
                }
            }
        };
    }
}

abstract class Base
{
    private $value;

    abstract public function __construct();

    public function run()
    {
        if ($this->value instanceof stdClass) {
            echo 13;
        }
    }
}
-----
<?php

namespace App\Conditions;

use stdClass;

function outside(stdClass $value)
{
    if ($value instanceof stdClass) {
        echo 0;
    }
}

class Other
{
}

final class Holder
{
    private stdClass $typed;
    private stdClass|Other $union;
    private $inferred;
    private $text = '';
    public stdClass $open;

    public function __construct(stdClass $param)
    {
        echo 'start';
        strlen('x');
        $local = 1;
        $this->text .= 'more';
        $this->typed = $param;
        $this->union = $param;
        $this->inferred = new stdClass();
        $this->open = $param;
    }

    public function conditions(bool $flag, stdClass $value, self $other, string $class, string $name)
    {
        if ($this->typed instanceof stdClass) {
            echo 1;
        } else {
            echo 2;
        }
        if (!$flag) {
            echo 3;
        }
        if ($flag) {
            echo 4;
        }
        if ($value instanceof stdClass) {
            echo 5;
        }
        if ($other->typed instanceof stdClass) {
            echo 6;
        }
        if ($this->typed instanceof $class) {
            echo 7;
        }
        if ($this->{$name} instanceof stdClass) {
            echo 8;
        }
        if ($this->union instanceof stdClass) {
            echo 9;
        }
        echo 10;
        echo 11;
    }

    public function anonymous()
    {
        return new class (new stdClass()) {
            public function __construct(private stdClass $inner)
            {
            }

            public function run()
            {
                echo 12;
            }
        };
    }
}

abstract class Base
{
    private $value;

    abstract public function __construct();

    public function run()
    {
        if ($this->value instanceof stdClass) {
            echo 13;
        }
    }
}
