<?php
namespace PrerequisiteBoundaryValidation;

function orderedNativeOffsets(): void
{
    $shape = ['z' => ['second' => 2, 'first' => 1], 'a' => 3];
    if (in_array(1, $shape['z'], true)) {
        echo 'found';
    }
}

class ByReferenceProperty
{
    public int $value = 1;

    public function access(): void
    {
        $reference =& $this->value;
        $reference = 2;
        echo $this->value;
    }
}

class MissingParent
{
    public function value(parent $value): void
    {
        \PHPStan\dumpType($value);
        \PHPStan\dumpType(new parent());
        \PHPStan\dumpType(parent::class);
    }
}
