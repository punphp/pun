<?php

namespace App;

use Foo\Bar as Baz;
use \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass;

new \Other\Bar();
new OldClass();
new \Foo\Qux();
/** @var \Iterator */
$x = 1;
/** @var Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass */
$y = 2;
// trailing
-----
<?php

namespace App;

new \Other\Bar();
new \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass();
new \Foo\Qux();
/** @var \Iterator */
$x = 1;
/** @var Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass */
$y = 2;
// trailing
