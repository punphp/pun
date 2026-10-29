<?php

namespace App;

/** @var \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass */
use Foo\Bar;
-----
<?php

namespace App;

/** @var OldClass */
use Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass;
