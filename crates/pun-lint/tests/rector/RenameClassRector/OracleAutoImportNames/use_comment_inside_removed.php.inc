<?php

namespace App;

use Foo\Unused /* c */;
use Foo\Used;

class A
{
    public function run()
    {
        new Used();
        new \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass();
    }
}
-----
<?php

namespace App;

use Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass;
use Foo\Used;

class A
{
    public function run()
    {
        new Used();
        new NewClass();
    }
}
