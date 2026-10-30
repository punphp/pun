<?php

/** @var \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass */
namespace App {
    // comment
    use Foo\Bar;
}
-----
<?php

/** @var OldClass */
namespace App {
    // comment
    use Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass;
}
