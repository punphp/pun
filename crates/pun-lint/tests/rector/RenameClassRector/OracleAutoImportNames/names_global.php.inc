<?php

use Doctrine\ORM\Mapping as ORM;
use Iterator;

namespace\Foo::x();
Foo\Bar::x();
foo();
$x::y();
new $x();
X::{$c};
$o = new class extends \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass implements \Iterator {};

class T
{
    use A, B {
        A::f insteadof B;
    }
}

class K extends \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass
{
    public function f()
    {
        new static();
        self::X;
        parent::f();
    }
}

/**
 * @ORM(x)
 * @var \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass
 */
function g()
{
}

/**
 * @var Iterator\X
 * @var Foo
 * @return \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass
 */
function h()
{
}
-----
<?php

use Foo\Bar;
use Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass;
use Doctrine\ORM\Mapping as ORM;
use Iterator;

Foo::x();
Bar::x();
foo();
$x::y();
new $x();
X::{$c};
$o = new class extends NewClass implements Iterator {};

class T
{
    use A, B {
        A::f insteadof B;
    }
}

class K extends NewClass
{
    public function f()
    {
        new static();
        self::X;
        parent::f();
    }
}

/**
 * @ORM(x)
 * @var NewClass
 */
function g()
{
}

/**
 * @var Iterator\X
 * @var Foo
 * @return NewClass
 */
function h()
{
}
