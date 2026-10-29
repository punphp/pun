<?php

namespace App;

class A
{
    /**
     * @var \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\Foo2\SomeShort
     */
    public $a;

    /**
     * @var \SomeShort
     */
    public $b;

    public function run()
    {
        new \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass();
    }
}
-----
<?php

namespace App;

use Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass;
use SomeShort;

class A
{
    /**
     * @var \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\Foo2\SomeShort
     */
    public $a;

    /**
     * @var SomeShort
     */
    public $b;

    public function run()
    {
        new NewClass();
    }
}
