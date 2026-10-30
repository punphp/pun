<?php

namespace App;

echo FOO;
echo \BAR;
new \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass();
-----
<?php

namespace App;

use Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass;

echo FOO;
echo \BAR;
new NewClass();
