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

use Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass;
use Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass;

class A
{
    public function f(): NewClass
    {
    }

    /** @var OldClass */

}

/**
 * @var OldClass
 */

