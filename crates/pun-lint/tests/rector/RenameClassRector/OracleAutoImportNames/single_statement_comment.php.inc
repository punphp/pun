<?php
// c
new \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass();
-----
<?php

use Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass;

// c
new NewClass();
