<?php

namespace App;

class A
{
    public function f(): \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass
    {
    }

    /** @var \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\SomeFinalClass */
    /** @var \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\SomeNonFinalClass */
    // tail
}

/** @var \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\DuplicatedClass */ /** @var \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\TwigFilter */
-----
<?php

namespace App;

use Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass;
use Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\SomeNonFinalClass;

class A
{
    public function f(): NewClass
    {
    }

    /** @var \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\SomeFinalClass */
    /** @var SomeNonFinalClass */
    // tail

}

/** @var \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\DuplicatedClass */ /** @var \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\TwigFilter */
