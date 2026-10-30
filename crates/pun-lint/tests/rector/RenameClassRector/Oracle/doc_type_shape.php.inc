<?php

namespace App\Types;

use Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass;

final class Holder
{
    /**
     * @param array{a: OldClass, ...<int, OldClass>} $c
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
     * @param array{a: \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass, ...<int, \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass>} $c
     */
    public function run($c)
    {
    }
}
