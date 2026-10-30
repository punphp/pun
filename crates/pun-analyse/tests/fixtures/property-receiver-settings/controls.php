<?php

declare(strict_types=1);

namespace PropertyReceiverControls;

class Target
{
    public int $value = 1;
    public static int $shared = 1;
    protected int $hidden = 1;
    protected static int $hiddenStatic = 1;
}

function probe(Target $ordinary, ?Target $nullable, string $class): void
{
    echo $ordinary->hidden;
    echo $ordinary::$hiddenStatic;

    echo Target::$hiddenStatic;
    echo Target::$value;
    echo Target::$missingNamedClass;

    echo $ordinary->value;
    echo ($ordinary)->value;
    echo $nullable?->value;
    echo $ordinary::$shared;
    echo ($ordinary)::$shared;
    echo $class::$uncheckedStringClass;

    // TypesAssignedToPropertiesRule registers at level 3 independently of checkThisOnly.
    $ordinary->value = 'not an int';
}
