<?php

declare(strict_types=1);

/**
 * @var \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass
 */
namespace App;

use Foo\Bar;
-----
<?php

declare(strict_types=1);

/**
 * @var OldClass
 */
namespace App;

use Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass;
