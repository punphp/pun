<?php

namespace Rector\Tests\TypeDeclaration\Rector\ClassMethod\NarrowObjectReturnTypeRector\Oracle;

use Rector\Tests\TypeDeclaration\Rector\ClassMethod\NarrowObjectReturnTypeRector\Source\Talk;
use Rector\Tests\TypeDeclaration\Rector\ClassMethod\NarrowObjectReturnTypeRector\Source\ConferenceTalkExtended;

abstract class GenericTalkFactory
{
    /**
     * @return \ArrayObject<int, Talk>
     */
    abstract public function build(): \ArrayObject;
}

final class SkipGenericParentDoc extends GenericTalkFactory
{
    public function build(): \ArrayObject
    {
        return new ParentArrayObject();
    }
}

class ParentArrayObject extends \ArrayObject
{
}
