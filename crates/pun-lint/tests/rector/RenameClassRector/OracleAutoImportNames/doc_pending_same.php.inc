<?php

namespace App;

class A
{
    public function run()
    {
        new \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass();

        return new \Foo\Thing();
    }

    /**
     * @var \Foo\Thing
     */
    public $a;
}
-----
<?php

namespace App;

use Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass;
use Foo\Thing;

class A
{
    public function run()
    {
        new NewClass();

        return new Thing();
    }

    /**
     * @var \Foo\Thing
     */
    public $a;
}
