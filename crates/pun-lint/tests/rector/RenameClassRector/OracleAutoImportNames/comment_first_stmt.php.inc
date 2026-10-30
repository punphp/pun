<?php

namespace App\C2;

/** a doc */
$a = new \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass();
-----
<?php

namespace App\C2;

use Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass;

/** a doc */
$a = new NewClass();
