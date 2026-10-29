<?php

namespace App\R;

use Foo\Kept;
use Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass;
use Foo\Kept2;

new Kept();
new Kept2();
$a = new OldClass();
-----
<?php

namespace App\R;

use Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass;
use Foo\Kept;
use Foo\Kept2;

new Kept();
new Kept2();
$a = new NewClass();
