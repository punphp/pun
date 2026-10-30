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
