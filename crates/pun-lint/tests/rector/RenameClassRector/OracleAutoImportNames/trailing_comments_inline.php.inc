<?php

namespace App;

new \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass(); // a
// b
/* c */
-----
<?php

namespace App;

use Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass;

new NewClass(); // a
// b
/* c */
