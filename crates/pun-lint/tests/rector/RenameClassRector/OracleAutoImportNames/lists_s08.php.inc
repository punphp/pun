<?php

namespace App;

// a
use Foo\Unused;
// b
use Foo\Unused2;

new \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass();
-----
<?php

namespace App;

// a
use Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass;
new NewClass();
