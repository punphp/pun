<?php

// Rector reads the untyped property's type from the constructor's assignment; Pun knows no property's type, and
// leaves the call.

namespace Rector\Tests\CodingStyle\Rector\FuncCall\CallUserFuncToMethodCallRector\Fixture;

final class SomeClass
{
    private $property;

    public function __construct(\stdClass $property)
    {
        $this->property = $property;
    }

    public function run()
    {
        $result = \call_user_func([$this->property, 'method'], $args);
    }
}

?>
