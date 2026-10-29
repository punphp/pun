<?php

namespace App\Rel;

use function Foo\helper as h;
use const Foo\LIMIT as L;
use Foo\Space as Sp;

h();
echo L;
new Sp\Inner();
new namespace\Local();
$a = new \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass();
-----
<?php

namespace App\Rel;

use Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass;
use function Foo\helper as h;
use const Foo\LIMIT as L;
use Foo\Space as Sp;

h();
echo L;
new Sp\Inner();
new namespace\Local();
$a = new NewClass();
