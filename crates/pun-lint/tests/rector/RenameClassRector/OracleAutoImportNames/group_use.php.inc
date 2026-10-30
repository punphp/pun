<?php

namespace App\G;

use Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\{OldClass, NewClassWithoutTypo};

$a = new OldClass();
$b = new NewClassWithoutTypo();
-----
<?php

namespace App\G;

use Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass;
use Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\{OldClass, NewClassWithoutTypo};

$a = new NewClass();
$b = new NewClassWithoutTypo();
