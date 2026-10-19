<?php

// Pun: a class that extends another stays, since the syntax cannot see whether the parent declares the method.

namespace Rector\Tests\Privatization\Rector\ClassMethod\PrivatizeFinalClassMethodRector\Fixture;

use Rector\Tests\Privatization\Rector\ClassMethod\PrivatizeFinalClassMethodRector\Source\AbstractClassWithProtectedClassMethod;

final class ChangeParentWithoutCare extends AbstractClassWithProtectedClassMethod
{
    protected function getSome()
    {
    }
}

?>
