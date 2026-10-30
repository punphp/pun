<?php

namespace App;

echo FOO;
echo \BAR;
new \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass();
-----
<?php

namespace App;

echo FOO;
echo \BAR;
new \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass();
