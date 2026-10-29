<?php

namespace Rector\Tests\TypeDeclaration\Rector\ClassMethod\NarrowObjectReturnTypeRector\Oracle;

use Rector\Tests\TypeDeclaration\Rector\ClassMethod\NarrowObjectReturnTypeRector\Source\ConferenceTalk;

final class TwoReturnsSameClass
{
    public function create(bool $flag): object
    {
        if ($flag) {
            return new ConferenceTalk();
        }

        return new ConferenceTalk();
    }
}
-----
<?php

namespace Rector\Tests\TypeDeclaration\Rector\ClassMethod\NarrowObjectReturnTypeRector\Oracle;

use Rector\Tests\TypeDeclaration\Rector\ClassMethod\NarrowObjectReturnTypeRector\Source\ConferenceTalk;

final class TwoReturnsSameClass
{
    public function create(bool $flag): \Rector\Tests\TypeDeclaration\Rector\ClassMethod\NarrowObjectReturnTypeRector\Source\ConferenceTalk
    {
        if ($flag) {
            return new ConferenceTalk();
        }

        return new ConferenceTalk();
    }
}
