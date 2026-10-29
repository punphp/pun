<?php

namespace App;

class A
{
    public function run()
    {
        new \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass();

        return \Foo\thing();
    }

    /**
     * @var \Other\Thing
     */
    public $a;
}
-----
<?php

namespace App;

use Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass;
use function Foo\thing;

class A
{
    public function run()
    {
        new NewClass();

        return thing();
    }

    /**
     * @var \Other\Thing
     */
    public $a;
}
