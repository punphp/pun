<?php

namespace App;

final class Item {}

class Base
{
    protected $typed;
}

trait Writes
{
    public function reset()
    {
        $this->counter++;
    }
}

final class C extends Base
{
    use Writes;

    public $label;
    public $number;
    public $ratio;
    public $enabled;
    public $list;
    public $either;
    protected $typed;
    private $counter;
    private $values = [];
    private $first;
    private $nested;
    private $viaClosure;

    public function __construct(int $counter, array $values, Item $item, ?Item $nullable)
    {
        $this->label = 'abc';
        $this->number = 5;
        $this->ratio = 1.5;
        $this->enabled = true;
        $this->list = [1, 2];
        $this->either = $nullable;
        $this->typed = $counter;
        $this->counter = $counter;
        $this->values[] = $item;
        $this->first = $item;
        $this->first = $item;
        if ($counter > 1) {
            $this->nested = 'x';
        } else {
            $this->nested = 'y';
        }
        $f = function () use ($item) {
            $this->viaClosure = $item;
        };
    }

    public function change(?Item $item)
    {
        $this->first = $item;
    }
}

$anonymous = new class(1) {
    private $value;

    public function __construct(int $value)
    {
        $this->value = $value;
    }
};
-----
<?php

namespace App;

final class Item {}

class Base
{
    protected $typed;
}

trait Writes
{
    public function reset()
    {
        $this->counter++;
    }
}

final class C extends Base
{
    use Writes;

    /**
     * @var 'abc'
     */
    public $label;
    /**
     * @var int
     */
    public $number;
    /**
     * @var 1.5
     */
    public $ratio;
    /**
     * @var true
     */
    public $enabled;
    public $list;
    /**
     * @var null|\App\Item
     */
    public $either;
    protected $typed;
    private $counter;
    private $values = [];
    private $first;
    private string $nested;
    private $viaClosure;

    public function __construct(int $counter, array $values, Item $item, ?Item $nullable)
    {
        $this->label = 'abc';
        $this->number = 5;
        $this->ratio = 1.5;
        $this->enabled = true;
        $this->list = [1, 2];
        $this->either = $nullable;
        $this->typed = $counter;
        $this->counter = $counter;
        $this->values[] = $item;
        $this->first = $item;
        $this->first = $item;
        if ($counter > 1) {
            $this->nested = 'x';
        } else {
            $this->nested = 'y';
        }
        $f = function () use ($item) {
            $this->viaClosure = $item;
        };
    }

    public function change(?Item $item)
    {
        $this->first = $item;
    }
}

$anonymous = new class(1) {
    private int $value;

    public function __construct(int $value)
    {
        $this->value = $value;
    }
};
