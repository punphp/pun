<?php

namespace App;

class A
{
    public function run()
    {
        new \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass();

        return new \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\Foo2\Storage();
    }

    /**
     * @var \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\Foo2\Bar\Storage
     */
    public $b;
}
-----
<?php

namespace App;

use Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass;
use Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\Foo2\Storage;

class A
{
    public function run()
    {
        new NewClass();

        return new Storage();
    }

    /**
     * @var \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\Foo2\Bar\Storage
     */
    public $b;
}
