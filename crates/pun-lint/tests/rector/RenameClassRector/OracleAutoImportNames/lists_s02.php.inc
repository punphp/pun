<?php

namespace App {
    new \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass();
}
-----
<?php

namespace App {
    use Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass;

    new NewClass();
}
