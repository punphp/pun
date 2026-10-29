<?php

namespace App;

// c
/** @var Bar|\Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass */
use Foo\Bar;

new \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass();
-----
<?php

namespace App;

// c
/** @var Bar|\Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass */
use Foo\Bar;

new \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass();
