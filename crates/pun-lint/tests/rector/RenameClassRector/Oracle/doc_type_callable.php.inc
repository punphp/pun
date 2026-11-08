<?php

namespace App\Types;

use Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass;

final class Holder
{
    /**
     * @param callable(OldClass): OldClass $c
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
     * @param callable(\Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass): \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass $c
     */
    public function run($c)
    {
    }
}
