<?php

namespace App\Types;

use Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass;

final class Holder
{
    /**
     * @template T of OldClass
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
     * @template T of \Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\NewClass
     */
    public function run($c)
    {
    }
}
