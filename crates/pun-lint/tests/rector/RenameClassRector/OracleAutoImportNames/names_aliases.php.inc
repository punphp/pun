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

use Other\Bar;
use Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass;
use Foo\Qux;
use Iterator;
new Bar();
new NewClass();
new Qux();
/** @var Iterator */
$x = 1;
/** @var NewClass */
$y = 2;
// trailing
