<?php

namespace App;

use Doctrine\ORM\Mapping as ORM;

final class Item {}

final class Cases
{
    private $count = 0;
    private $name;
    private $items = [];
    private $nullable = null;
    private $union;
    private $mixedParam;
    protected $protected;
    public $public;
    private $dynamic;
    /** @var Item|null */
    private $item;
    /** @var string The name */
    private $described;
    private static $static;
    /** @ORM\Column(type="string") */
    private $column;
    #[ORM\Column]
    private $attributeColumn;
    private $neverAssigned;
    private $neverAssignedDefault = 'x';
    private $closure;
    private $anonymous;

    public function __construct(int $count, $mixedParam)
    {
        $this->count = $count;
        $this->mixedParam = $mixedParam;
        $this->protected = 1;
        $this->public = 1;
        $this->item = new Item();
        $this->closure = function () {};
        $this->anonymous = new class {};
    }

    public function run(string $name, bool $flag): void
    {
        $this->name = $name;
        $this->items[] = $name;
        $this->nullable = $name;
        $this->union = $flag ? 1 : 'one';
        $this->dynamic = 1;
        $this->described = $name;
        self::$static = $flag;
        $this->column = $name;
        $this->attributeColumn = $name;
    }
}
-----
<?php

namespace App;

use Doctrine\ORM\Mapping as ORM;

final class Item {}

final class Cases
{
    private int $count = 0;
    private ?string $name = null;
    private array $items = [];
    private ?string $nullable = null;
    private $union;
    private $mixedParam;
    protected $protected;
    public $public;
    private ?int $dynamic = null;
    private \App\Item $item;
    /** @var string The name */
    private ?string $described = null;
    private static ?bool $static = null;
    /** @ORM\Column(type="string") */
    private $column;
    #[ORM\Column]
    private $attributeColumn;
    private $neverAssigned;
    private string $neverAssignedDefault = 'x';
    private \Closure $closure;
    private object $anonymous;

    public function __construct(int $count, $mixedParam)
    {
        $this->count = $count;
        $this->mixedParam = $mixedParam;
        $this->protected = 1;
        $this->public = 1;
        $this->item = new Item();
        $this->closure = function () {};
        $this->anonymous = new class {};
    }

    public function run(string $name, bool $flag): void
    {
        $this->name = $name;
        $this->items[] = $name;
        $this->nullable = $name;
        $this->union = $flag ? 1 : 'one';
        $this->dynamic = 1;
        $this->described = $name;
        self::$static = $flag;
        $this->column = $name;
        $this->attributeColumn = $name;
    }
}
