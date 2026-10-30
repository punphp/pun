<?php

namespace App;

class A
{
    public function run()
    {
        new \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass();

        return \Foo\someservicefunc() + \Foo\somenonfinalclass();
    }

    /**
     * @var \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\SomeNonFinalClass
     */
    public $b;
}
-----
<?php

namespace App;

use Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass;
use function Foo\someservicefunc;
use function Foo\somenonfinalclass;

class A
{
    public function run()
    {
        new NewClass();

        return someservicefunc() + somenonfinalclass();
    }

    /**
     * @var \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\SomeNonFinalClass
     */
    public $b;
}
