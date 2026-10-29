<?php

namespace App;

new \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass(); // a
// b
/* c */
-----
<?php

namespace App;

new \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass(); // a
// b
/* c */
