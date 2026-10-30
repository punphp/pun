<?php

namespace App\Used;

use Foo\SeeTarget;
use Foo\InlineSee;
use Foo\InlineUses;
use Foo\Generic;
use Foo\Nested;
use Foo\ConstHolder;
use Foo\Leading;
use Foo\Aliased\Space;
use Foo\Unused;
use Foo\Method;

/**
 * @see SeeTarget::run()
 * Read {@see InlineSee} and {@uses Space\Inner::x} and {@uses \Foo\Global} and {@see 1bad} and {@see}
 * @customtag Generic
 * @other Leading some words
 * @Nested\Thing(@Nested\Inner, value="@Not")
 * @method Method run()
 */
final class Docs
{
    /**
     * @var ConstHolder::VALUE
     */
    public $a;
}

$o = new \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass();
-----
<?php

namespace App\Used;

use Foo\SeeTarget;
use Foo\InlineSee;
use Foo\Generic;
use Foo\Nested;
use Foo\ConstHolder;
use Foo\Leading;
use Foo\Aliased\Space;
use Foo\Method;

/**
 * @see SeeTarget::run()
 * Read {@see InlineSee} and {@uses Space\Inner::x} and {@uses \Foo\Global} and {@see 1bad} and {@see}
 * @customtag Generic
 * @other Leading some words
 * @Nested\Thing(@Nested\Inner, value="@Not")
 * @method Method run()
 */
final class Docs
{
    /**
     * @var ConstHolder::VALUE
     */
    public $a;
}

$o = new \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass();
