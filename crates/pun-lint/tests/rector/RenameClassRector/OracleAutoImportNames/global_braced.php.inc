<?php

namespace {
    $a = new \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass();
}
-----
<?php

namespace {
    use Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass;

    $a = new NewClass();
}
