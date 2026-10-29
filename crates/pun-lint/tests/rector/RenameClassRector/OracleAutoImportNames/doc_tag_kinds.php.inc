<?php

namespace App;

/**
 * @phpstan-type Foo \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass
 * @phpstan-import-type Bar from \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass
 * @template T of \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass
 * @phpstan-assert \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass $a
 * @mixin \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass
 * @see \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass
 * @throws \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass
 * @extends \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass<int>
 * @method static \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass foo()
 * @property \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass $x
 * @uses \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass
 */
class A
{
    public function run()
    {
        new \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass();
    }
}
-----
<?php

namespace App;

use Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass;

/**
 * @phpstan-type Foo NewClass
 * @phpstan-import-type Bar from NewClass
 * @template T of \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass
 * @phpstan-assert NewClass $a
 * @mixin NewClass
 * @see \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass
 * @throws NewClass
 * @extends NewClass<int>
 * @method static NewClass foo()
 * @property NewClass $x
 * @uses \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass
 */
class A
{
    public function run()
    {
        new NewClass();
    }
}
