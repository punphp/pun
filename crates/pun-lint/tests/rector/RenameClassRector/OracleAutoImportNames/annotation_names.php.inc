<?php

/**
 * @Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass()
 * @Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClassWithoutTypo(  key = "v",   other=1)
 * @Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\SomeFinalClass some words
 */
final class A
{
}
-----
<?php

use Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass;
use Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClassWithoutTypo;
use Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\SomeFinalClass;

/**
 * @NewClass
 * @NewClassWithoutTypo(key="v", other=1)
 * @SomeFinalClass
 */
final class A
{
}
