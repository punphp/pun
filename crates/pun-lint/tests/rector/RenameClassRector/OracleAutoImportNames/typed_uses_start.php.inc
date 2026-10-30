<?php

namespace App\T2;

use function Foo\a, Foo\b;

b();
$o = new \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass();
-----
<?php

namespace App\T2;

use Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass;
use function Foo\b;

b();
$o = new NewClass();
