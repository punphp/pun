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

use Foo\Bar;

/**
 * @see Baz::qux()
 * @var Quux::CONST
 * @Doctrine\ORM\Mapping\Column(type="Corge::X")
 * @return \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass
 */
function f()
{
    new Bar\Baz\Qux();
}
