<?php

namespace App\T;

use function Foo\a, Foo\b;
use const Foo\{X, Y};

a();
echo X;
$o = new \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass();
-----
<?php

namespace App\T;

use Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass;
use function Foo\a;
use const Foo\{X, Y};

a();
echo X;
$o = new NewClass();
