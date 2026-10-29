<?php

namespace {
    new \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass();
    new \Foo\Bar();
}
-----
<?php

namespace {
    use Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass;
    use Foo\Bar;

    new NewClass();
    new Bar();
}
