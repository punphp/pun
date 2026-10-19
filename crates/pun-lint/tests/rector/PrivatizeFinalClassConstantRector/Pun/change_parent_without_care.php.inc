<?php

// Pun: a class that extends another stays, since the syntax cannot see whether the parent declares the member.

namespace Rector\Tests\Privatization\Rector\ClassConst\PrivatizeFinalClassConstantRector\Fixture;

use Rector\Tests\Privatization\Rector\ClassConst\PrivatizeFinalClassConstantRector\Source\AbstractClassWithProtectedClassConst;

final class ChangeParentWithoutCare extends AbstractClassWithProtectedClassConst
{
    protected const PROTECTED_LOCAL_CONST = 'local';
}
?>
