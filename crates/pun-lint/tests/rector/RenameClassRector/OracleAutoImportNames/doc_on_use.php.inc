<?php

namespace App;

/** @var \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass $x */
use Foo\Bar;

new Bar();
-----
<?php

namespace App;

/** @var OldClass $x */
use Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass;
use Foo\Bar;

new Bar();
