<?php

new \DateTime();
/** @var \Iterator */
$a = 1;
new \Foo\Bar();
new \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass();
-----
<?php

use Foo\Bar;
use Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass;

new \DateTime();
/** @var \Iterator */
$a = 1;
new Bar();
new NewClass();
