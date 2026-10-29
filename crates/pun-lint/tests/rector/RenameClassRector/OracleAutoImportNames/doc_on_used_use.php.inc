<?php

namespace App;

/** @var \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass|Bar */
use Foo\Bar;
-----
<?php

namespace App;

/** @var OldClass|Bar */
use Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass;
use Foo\Bar;
