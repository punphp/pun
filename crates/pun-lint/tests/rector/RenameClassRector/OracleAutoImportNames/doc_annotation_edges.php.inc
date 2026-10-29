<?php

namespace App;

/**
 * @\Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass()
 * @\Foo\A(true::X)
 * @\Foo\B({@\Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass: "x"})
 */
class A
{
}
-----
<?php

namespace App;

use Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass;

/**
 * @OldClass
 * @\Foo\A(true::X)
 * @\Foo\B({@\Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass: "x"})
 */
class A
{
}
