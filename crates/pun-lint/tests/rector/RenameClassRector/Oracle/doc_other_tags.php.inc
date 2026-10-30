<?php

namespace App;

use Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass;

/**
 * @method OldClass make(OldClass $a, int &...$b, $c = 'it\'s\\', array $d = [1, 'k' => "v\$", null], $e = \PHP_EOL, $f = 1.5, $g = Foo::BAR, $h = true) Makes one.
 * @method OldClass broken(OldClass $a, int &...$b, $c = 'it''s', array $d = [1, 'k' => "v\$", null], $e = \PHP_EOL, $f = 1.5, $g = Foo::BAR, $h = true) Makes one.
 * @method static OldClass build()
 * @method static create()
 * @method plain()
 * @method OldClass generic<T of OldClass, U as \Countable = int, V>(T $t) with templates
 * @phpstan-type Shape object{a: OldClass, 'b'?: int}
 * @psalm-type Cond = (T is OldClass ? int : string)
 * @phpstan-import-type Alias from OldClass as Renamed
 * @phpstan-import-type Other from OldClass
 * @phpstan-assert OldClass $x
 * @phpstan-assert-if-true !=OldClass $this->prop the property
 * @psalm-assert OldClass $x->method() called
 * @template T of OldClass = OldClass the template
 * @template U super OldClass
 * @param OldClass[$k] $p offset access
 * @param (callable(): int)[0] $q
 * @param ($p is OldClass ? int : string) $r
 * @param ?OldClass[int] $s
 */
class A
{
}
-----
<?php

namespace App;

/**
 * @method \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass make(\Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass $a, int &...$b, $c = 'it\'s\\', array $d = [1, 'k' => "v\\\$", null], $e = \PHP_EOL, $f = 1.5, $g = Foo::BAR, $h = true) Makes one.
 * @method OldClass broken(OldClass $a, int &...$b, $c = 'it''s', array $d = [1, 'k' => "v\$", null], $e = \PHP_EOL, $f = 1.5, $g = Foo::BAR, $h = true) Makes one.
 * @method static \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass build()
 * @method static create()
 * @method plain()
 * @method \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass generic<T of \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass, U as \stdClass, V>(T $t) with templates
 * @phpstan-type Shape object{a: \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass, 'b'?: int}
 * @psalm-type Cond (T is \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass ? int : string)
 * @phpstan-import-type Alias from \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass as Renamed
 * @phpstan-import-type Other from \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass
 * @phpstan-assert \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass $x
 * @phpstan-assert-if-true !=\Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass $this->prop the property
 * @psalm-assert \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass $x->method() called
 * @template T of \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass the template
 * @template U super OldClass
 * @param OldClass[$k] $p offset access
 * @param (callable(): int)[0] $q
 * @param ($p is \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass ? int : string) $r
 * @param ?\Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass[int] $s
 */
class A
{
}
