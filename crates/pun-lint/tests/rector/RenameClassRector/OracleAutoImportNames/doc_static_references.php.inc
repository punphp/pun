<?php

use Foo\Bar;

/**
 * @see Baz::qux()
 * @var Quux::CONST
 * @Doctrine\ORM\Mapping\Column(type="Corge::X")
 * @return \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass
 */
function f()
{
    new Bar\Baz\Qux();
}
-----
<?php

use Foo\Bar\Baz\Qux;
use Doctrine\ORM\Mapping\Column;
use Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass;
/**
 * @see Baz::qux()
 * @var Quux::CONST
 * @Column(type="Corge::X")
 * @return NewClass
 */
function f()
{
    new Qux();
}
