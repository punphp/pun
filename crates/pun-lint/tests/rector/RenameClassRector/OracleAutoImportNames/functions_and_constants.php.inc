<?php

namespace App\Functions;

use function Foo\Bar\used;
use const Foo\Bar\USED;

$a = new \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass();
\Foo\Bar\baz();
echo \Foo\Bar\QUX;
used();
echo USED;
-----
<?php

namespace App\Functions;

use Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass;
use const Foo\Bar\QUX;
use function Foo\Bar\baz;
use function Foo\Bar\used;
use const Foo\Bar\USED;

$a = new NewClass();
baz();
echo QUX;
used();
echo USED;
