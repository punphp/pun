<?php

namespace App;

use Foo\Bar;
use Foo\Unused;

class A
{
    /**
     * @Doctrine\ORM\Mapping\Column(name="abc def)
     */
    const X = 1;

    public function f(): \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass
    {
        return new Bar\Baz\Qux();
    }
}
