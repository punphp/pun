<?php

// head
/** @var \Foo\X */

new \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass();
-----
<?php

use Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass;

// head
/** @var \Foo\X */
new NewClass();
