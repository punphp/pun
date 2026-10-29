<?php

namespace App {
    use Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass;

    new OldClass();
    new \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\FirstNamespace\SomeServiceClass();
}
-----
<?php

namespace App {
    use Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass;
    use Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\SecondNamespace\SomeServiceClass;

    new NewClass();
    new SomeServiceClass();
}
