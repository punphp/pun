<?php

namespace App;

new \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass();
// one
/* two */
-----
<?php

namespace App;

use Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass;

new NewClass();
// one
/* two */
