<?php

namespace App;

class A
{
    /**
     * @var \Foo<*, \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass>
     */
    public $a;

    /**
     * @var \Foo<covariant \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass, contravariant int>
     */
    public $b;

    /**
     * @param \Foo<*>|\Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass $c
     */
    public function run($c)
    {
    }
}
-----
<?php

namespace App;

use Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass;

class A
{
    /**
     * @var \Foo<*, NewClass>
     */
    public $a;

    /**
     * @var \Foo<covariant NewClass, contravariant int>
     */
    public $b;

    /**
     * @param \Foo<*>|NewClass $c
     */
    public function run($c)
    {
    }
}
