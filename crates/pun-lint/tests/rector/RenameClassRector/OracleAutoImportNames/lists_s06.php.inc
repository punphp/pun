<?php

namespace App;

// c
use Foo\Bar;

new \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass();
new Bar();
-----
<?php

namespace App;

// c
use Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass;
use Foo\Bar;

new NewClass();
new Bar();
