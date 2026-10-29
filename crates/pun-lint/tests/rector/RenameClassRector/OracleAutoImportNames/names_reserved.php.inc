<?php

namespace App;

/** @var \Foo\Mixed */
$a = 1;
new \Foo\Bar\Iterable();
new \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass();
-----
<?php

namespace App;

use Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass;

/** @var \Foo\Mixed */
$a = 1;
new \Foo\Bar\Iterable();
new NewClass();
