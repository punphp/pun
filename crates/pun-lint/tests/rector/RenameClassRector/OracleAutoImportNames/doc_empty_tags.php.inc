<?php

namespace App;

/**
 * @template
 * @template T of
 * @extends
 * @psalm-type X =
 * @return \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass
 */
class A
{
}
-----
<?php

namespace App;

use Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass;

/**
 * @template
 * @template T of
 * @extends
 * @psalm-type X =
 * @return NewClass
 */
class A
{
}
