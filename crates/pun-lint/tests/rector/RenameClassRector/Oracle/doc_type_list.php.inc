<?php

namespace App\Types;

use Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass;

final class Holder
{
    /**
     * @param OldClass[]|$this $c
     */
    public function run($c)
    {
    }
}
-----
<?php

namespace App\Types;

final class Holder
{
    /**
     * @param \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass[]|$this $c
     */
    public function run($c)
    {
    }
}
