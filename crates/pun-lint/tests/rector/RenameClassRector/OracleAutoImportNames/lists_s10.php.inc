<?php

declare(strict_types=1);

use Foo\Bar;

new \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass();
new Bar();
-----
<?php

declare(strict_types=1);

use Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass;
use Foo\Bar;

new NewClass();
new Bar();
