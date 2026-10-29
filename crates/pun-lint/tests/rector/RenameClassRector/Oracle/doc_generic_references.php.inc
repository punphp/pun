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

/**
 * @custom Bar::x
 * {@see}
 * {@seeFoo}
 * @return \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass
 */
function f()
{
}
