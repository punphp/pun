<?php

namespace App\N;

$a = new \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass();
// trailing comment
-----
<?php

namespace App\N;

use Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass;

$a = new NewClass();
// trailing comment
