<?php

namespace App;

/**
 * @Some\Annotation
 * ({
 *   "x"
 * })
 * @return \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass
 */
function f()
{
}

/**
 * @\Foo\Bar(x=1) some text
 * more text
 * even more
 * @return \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass
 */
function g()
{
}
-----
<?php

namespace App;

/**
 * @Some\Annotation
 * ({
 *   "x"
 * })
 * @return \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass
 */
function f()
{
}

/**
 * @\Foo\Bar(x=1) some text
 * more text
 * even more
 * @return \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass
 */
function g()
{
}
