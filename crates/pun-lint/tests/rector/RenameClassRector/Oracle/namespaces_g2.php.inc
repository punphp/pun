<?php

namespace App {
    use Foo\Unused;
}

namespace {
    use Foo\Unused2;

    new \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass();
}
-----
<?php

namespace App {
}

namespace {
    new \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass();
}
