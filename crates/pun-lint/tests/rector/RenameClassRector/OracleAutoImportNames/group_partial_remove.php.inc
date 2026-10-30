<?php

namespace App\GP;

use Foo\{Used, Unused};

new Used();
$a = new \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass();
-----
<?php

namespace App\GP;

use Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass;
use Foo\{Used, Unused};

new Used();
$a = new NewClass();
