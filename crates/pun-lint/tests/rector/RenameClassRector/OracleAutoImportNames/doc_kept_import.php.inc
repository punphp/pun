<?php

namespace App;

use Foo\Unused;
use Foo\Kept;

/** @var Kept */
$a = 1;
new \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass();
-----
<?php

namespace App;

use Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass;
use Foo\Kept;

/** @var Kept */
$a = 1;
new NewClass();
