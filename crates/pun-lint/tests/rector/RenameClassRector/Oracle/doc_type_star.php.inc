<?php

namespace App\Types;

use Rector\Tests\Renaming\Rector\Name\RenameClassRector\Source\OldClass;

final class Holder
{
    /**
     * @param OldClass::* $c
     */
    public function run($c)
    {
    }
}
