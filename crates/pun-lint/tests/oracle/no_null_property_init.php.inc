<?php
class B {
    public $b = NULL, $c = 1, $d = null;
    public ?int $e = null;
    var $f = \null;
    public static $g = null;
    protected $i = null; // comment
    public function __construct(public $j = null) {}
}
trait T {
    private $t = null;
}
class Hooked {
    public $x = null { set => $value; }
    public ?int $y = null { set => $value; }
}
-----
<?php
class B {
    public $b, $c = 1, $d;
    public ?int $e = null;
    var $f;
    public static $g;
    protected $i; // comment
    public function __construct(public $j = null) {}
}
trait T {
    private $t;
}
class Hooked {
    public $x { set => $value; }
    public ?int $y = null { set => $value; }
}
