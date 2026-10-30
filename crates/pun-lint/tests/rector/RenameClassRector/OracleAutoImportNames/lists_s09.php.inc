<?php

declare(strict_types=1);

namespace App;

new \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass();
-----
<?php

declare(strict_types=1);

namespace App;

use Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass;

new NewClass();
