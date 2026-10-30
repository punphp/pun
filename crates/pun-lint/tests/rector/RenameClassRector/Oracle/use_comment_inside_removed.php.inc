<?php

namespace App;

use Foo\Used;
use Foo\Unused /* c */;
use Foo\Other;

class A
{
    public function run()
    {
        new Used();
        new Other();
        new \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass();
    }
}
-----
<?php

namespace App;

use Foo\Used;
use Foo\Other;

class A
{
    public function run()
    {
        new Used();
        new Other();
        new \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass();
    }
}
