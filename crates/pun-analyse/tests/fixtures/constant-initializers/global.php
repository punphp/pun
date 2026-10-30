<?php

function globalCopies(): void
{
    $value = GLOBAL_CHAIN;
    \PHPStan\dumpType($value);
    $value = 'reassigned';
    \PHPStan\dumpType($value);
    \PHPStan\dumpType(GLOBAL_VALUE);
}

const GLOBAL_CHAIN = GLOBAL_VALUE + 2;
const GLOBAL_VALUE = 5.9 % 2.0;

class TruthyInitializerSource
{
    /** @var non-empty-mixed */
    public const TRUTHY = 1;

    /** @var mixed */
    public const ANY = 1;
}

define('TRUTHY_INITIALIZER', (new TruthyInitializerSource)::TRUTHY);
define('MIXED_INITIALIZER', (new TruthyInitializerSource)::ANY);
define('INITIALIZER_CONDITION', (new TruthyInitializerSource)::ANY);
const TRUTHY_INITIALIZER_ALIAS = TRUTHY_INITIALIZER;

// Initializer reflection joins both arms: restoring one falsey value must retain the other exclusions.
const TRUTHY_WITH_EMPTY_ARRAY = INITIALIZER_CONDITION ? TRUTHY_INITIALIZER_ALIAS : [];
const TRUTHY_WITH_FALSE = INITIALIZER_CONDITION ? TRUTHY_INITIALIZER_ALIAS : false;
const MIXED_WITH_EMPTY_ARRAY = INITIALIZER_CONDITION ? MIXED_INITIALIZER : [];

function truthyInitializerCopies(): void
{
    \PHPStan\dumpType(TRUTHY_INITIALIZER);
    \PHPStan\dumpType(TRUTHY_INITIALIZER_ALIAS);
    \PHPStan\dumpType(MIXED_INITIALIZER);
    \PHPStan\dumpType(TRUTHY_WITH_EMPTY_ARRAY);
    \PHPStan\dumpType(TRUTHY_WITH_FALSE);
    \PHPStan\dumpType(MIXED_WITH_EMPTY_ARRAY);
}

class TruthyInitializerConsumer
{
    public const VALUE = TRUTHY_INITIALIZER;
    public const ANY = MIXED_INITIALIZER;

    public static function inspect(): void
    {
        $truthy = self::VALUE;
        if (is_array($truthy)) {
            \PHPStan\dumpType($truthy);
        }
        $plain = self::ANY;
        if (is_array($plain)) {
            \PHPStan\dumpType($plain);
        }
    }
}
