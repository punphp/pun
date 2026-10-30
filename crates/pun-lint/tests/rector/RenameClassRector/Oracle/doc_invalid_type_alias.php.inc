<?php

namespace App;

class A
{
    /**
     * @phpstan-type Foo int<
     * @var \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass
     */
    public $a;
}

class B
{
    /**
     * @var \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass
     * @phpstan-type Foo int<
     */
    public $a;

    /**
     * @param \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass $a
     * @phpstan-type Foo int<
     */
    public function run($a)
    {
    }
}
-----
<?php

namespace App;

class A
{
    /**
     * @phpstan-type Foo int<
     * @var \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass
     */
    public $a;
}

class B
{
    /**
     * @var \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass
     * @phpstan-type Foo int<*/
    public $a;

    /**
     * @param \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass $a
     * @phpstan-type Foo int<*/
    public function run($a)
    {
    }
}
