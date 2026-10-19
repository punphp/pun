<?php

// Pun: a class that extends another stays, since the syntax cannot see whether the parent declares or uses the property.

namespace Rector\Tests\Privatization\Rector\Property\PrivatizeFinalClassPropertyRector\Fixture;

use Rector\Tests\Privatization\Rector\Property\PrivatizeFinalClassPropertyRector\Source\AbstractClassWithProtectedProperty;

final class ChangeParentWithoutCare extends AbstractClassWithProtectedProperty
{
    protected $differentValue = 1000;
}

?>
