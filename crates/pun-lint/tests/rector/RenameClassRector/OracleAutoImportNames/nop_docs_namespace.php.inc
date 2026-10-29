<?php

namespace App;

new \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass();

    /** @var \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\SomeFinalClass */
-----
<?php

namespace App;

use Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass;
use Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\SomeFinalClass;

new NewClass();

    /** @var SomeFinalClass */

