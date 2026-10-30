<?php
declare(strict_types=1); ?>
<?php
use Foo\Bar;

new Bar();
new \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass();
-----
<?php

declare(strict_types=1);

use Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass;
use Foo\Bar;
new Bar();
new NewClass();
