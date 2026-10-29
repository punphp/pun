<?php

namespace App;

/** @var \A\Foo */
$a = 1;
/** @var \B\Foo */
$b = 1;
new \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass();
-----
<?php

namespace App;

use Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass;

/** @var \A\Foo */
$a = 1;
/** @var \B\Foo */
$b = 1;
new NewClass();
