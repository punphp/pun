<?php

namespace  App\Spaces ;
$a = new \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass();
-----
<?php

namespace App\Spaces;

use Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass;

$a = new NewClass();
