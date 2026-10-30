<?php

namespace App;

class A
{
    /**
     * @method \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass foo($a = "x\u{200000}y")
     */
    public $b;
}
-----
<?php

namespace App;

use Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass;

class A
{
    /**
     * @method OldClass foo($a = "x�y")
     */
    public $b;
}
