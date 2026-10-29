<?php

namespace App;

/** @var Foo */
use Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass;

new OldClass();
-----
<?php

namespace App;

use Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass;

new NewClass();
