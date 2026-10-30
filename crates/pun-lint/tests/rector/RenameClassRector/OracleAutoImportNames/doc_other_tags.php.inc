<?php

namespace App;

/**
 * @method \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass make(\Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\SomeFinalClass $a, $c = "x") Makes one.
 * @method \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClassWithoutTypo generic<T of \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\SomeNonFinalClass>(T $t)
 * @phpstan-type Shape object{a: \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass}
 * @phpstan-import-type Alias from \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\SomeFinalClass
 * @phpstan-assert \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\SomeFinalClass $x
 * @template T of \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass = \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\SomeFinalClass
 * @param (\Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass)[int] $p
 */
class A
{
}
-----
<?php

namespace App;

use Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass;
use Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\SomeFinalClass;
use Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClassWithoutTypo;

/**
 * @method NewClass make(SomeFinalClass $a, $c = "x") Makes one.
 * @method NewClassWithoutTypo generic<T of \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\SomeNonFinalClass>(T $t)
 * @phpstan-type Shape object{a: NewClass}
 * @phpstan-import-type Alias from SomeFinalClass
 * @phpstan-assert SomeFinalClass $x
 * @template T of NewClass
 * @param NewClass[int] $p
 */
class A
{
}
