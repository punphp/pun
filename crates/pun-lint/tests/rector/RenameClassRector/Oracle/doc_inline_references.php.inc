<?php

namespace App;

use Foo\Bar;
use Foo\Baz;

class A
{
    /**
     * See {@link x} and {@see Bar} or {@uses} {@see } {@used-by  Baz::run()}.
     */
    public function run()
    {
        new \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass();
    }
}
-----
<?php

namespace App;

use Foo\Bar;
use Foo\Baz;

class A
{
    /**
     * See {@link x} and {@see Bar} or {@uses} {@see } {@used-by  Baz::run()}.
     */
    public function run()
    {
        new \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass();
    }
}
