<?php

namespace App\C;

// first comment
use Foo\Bar;

$a = new \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass();
new Bar();
-----
<?php

namespace App\C;

// first comment
use Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass;
use Foo\Bar;

$a = new NewClass();
new Bar();
