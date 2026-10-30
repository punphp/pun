<?php

namespace App;

class K
{
    use \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass, B {
        \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass::f insteadof B;
        B::g insteadof \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass;
        \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass::h as k;
    }
}

$o = new class {};
$p = new class() implements \Iterator {};
-----
<?php

namespace App;

class K
{
    use \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass, B {
        \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass::f insteadof B;
        B::g insteadof \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass;
        \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass::h as k;
    }
}

$o = new class {};
$p = new class() implements \Iterator {};
