<?php

/** file doc */

$a = new \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass();
-----
<?php

use Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass;

/** file doc */
$a = new NewClass();
