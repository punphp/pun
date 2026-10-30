<?php

namespace N;

#[\Attribute]
class Marker
{
}

trait HasTitle
{
    public string $title;
}

interface HasLabel
{
    public string $label { get; }
}

class Base
{
    use HasTitle;

    public $a;
    protected int $b = 1;
    private $c;
    public static $s;
    public readonly int $r;
    var $v;
    public $multi;

    public function __construct(public int $promoted = 0)
    {
    }
}

class Middle extends Base
{
}

class Child extends Middle implements HasLabel
{
    public $a;

    /**
     * Doc.
     */
    protected int $b = 2;

    public $c;

    public static $s;

    #[Marker]
    public readonly int $r;

    var $v;

    public $other, $multi;

    public int $promoted = 1;

    public string $title;

    public string $label = 'x';

    private $a2;

    #[Override]
    public $own;
}

class Grand extends Child
{
    #[\Override]
    public $a;

    #[\Marker] #[\Override]
    protected int $b = 3;

    private $c;
}

class Lone
{
    public $a;
}

$x = new class extends Base {
    public $a;
};
-----
<?php

namespace N;

#[\Attribute]
class Marker
{
}

trait HasTitle
{
    public string $title;
}

interface HasLabel
{
    public string $label { get; }
}

class Base
{
    use HasTitle;

    public $a;
    protected int $b = 1;
    private $c;
    public static $s;
    public readonly int $r;
    var $v;
    public $multi;

    public function __construct(public int $promoted = 0)
    {
    }
}

class Middle extends Base
{
}

class Child extends Middle implements HasLabel
{
    #[\Override]
    public $a;

    /**
     * Doc.
     */
    #[\Override]
    protected int $b = 2;

    public $c;

    #[\Override]
    public static $s;

    #[Marker]
    #[\Override]
    public readonly int $r;

    #[\Override]
    var $v;

    #[\Override]
    public $other, $multi;

    #[\Override]
    public int $promoted = 1;

    #[\Override]
    public string $title;

    public string $label = 'x';

    private $a2;

    #[Override]
    public $own;
}

class Grand extends Child
{
    #[\Override]
    public $a;

    #[\Marker] #[\Override]
    protected int $b = 3;

    private $c;
}

class Lone
{
    public $a;
}

$x = new class extends Base {
    public $a;
};
