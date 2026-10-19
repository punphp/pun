<?php

// Pun: the members come from BaseClass, which the syntax cannot see, so `static::` stays (Rector reads them from reflection).

namespace Rector\Tests\CodeQuality\Rector\Class_\ConvertStaticToSelfRector\Fixture;

use Rector\Tests\CodeQuality\Rector\Class_\ConvertStaticToSelfRector\Source\BaseClass;

final class FinalClassInheritedMembers extends BaseClass
{
    public function run(): void
    {
        static::SUCCESS;
        static::$parentProperty;
        static::parentMethod();
    }
}

?>
