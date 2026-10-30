<?php

namespace App;

use Foo\Bar;

/**
 * @custom Bar::x
 * {@see}
 * {@seeFoo}
 * @return \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass
 */
function f()
{
}
-----
<?php

namespace App;

use Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass;

/**
 * @custom Bar::x
 * {@see}
 * {@seeFoo}
 * @return NewClass
 */
function f()
{
}
