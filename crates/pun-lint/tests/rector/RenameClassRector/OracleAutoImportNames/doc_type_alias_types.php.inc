<?php

namespace App;

/**
 * @phpstan-type A1 = \Foo&\Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass
 * @phpstan-type A2 = array<int, \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass>
 * @phpstan-type A3 = array{a: int, ...<\Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass>}
 * @phpstan-type A4 = (?\Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass)['a']
 * @phpstan-type A5 = (callable(): \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass)['a']
 * @phpstan-type A6 = (\Foo|\Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass)['a']
 */
class A
{
}
-----
<?php

namespace App;

use Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass;

/**
 * @phpstan-type A1 \Foo&OldClass
 * @phpstan-type A2 array<int, OldClass>
 * @phpstan-type A3 array{a: int, ...<OldClass>}
 * @phpstan-type A4 (?OldClass)['a']
 * @phpstan-type A5 (callable(): OldClass)['a']
 * @phpstan-type A6 (\Foo|OldClass)['a']
 */
class A
{
}
