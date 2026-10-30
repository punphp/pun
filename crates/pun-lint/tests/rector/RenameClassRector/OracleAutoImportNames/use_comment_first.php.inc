<?php

// c
use Foo\Bar;

new Bar();
new \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass();
-----
<?php

// c
use Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass;
use Foo\Bar;

new Bar();
new NewClass();
