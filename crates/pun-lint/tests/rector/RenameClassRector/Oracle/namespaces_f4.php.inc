<?php

namespace App {
    use Foo\Unused;

    new \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass();
}
-----
<?php

namespace App {
    new \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass();
}
