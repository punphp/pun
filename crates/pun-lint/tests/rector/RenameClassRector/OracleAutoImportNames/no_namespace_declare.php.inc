<?php

declare(strict_types=1);

// leading
$a = new \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass();
-----
<?php

declare(strict_types=1);

use Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass;

// leading
$a = new NewClass();
