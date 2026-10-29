<?php

namespace App;

class A
{
    public function f(): \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass
    {
    }

    /** @var \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass */
}

/**
 * @var \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass
 */
-----
<?php

namespace App;

class A
{
    public function f(): \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass
    {
    }

    /** @var \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass */
}

/**
 * @var \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass
 */
