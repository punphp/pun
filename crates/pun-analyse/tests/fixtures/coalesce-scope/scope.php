<?php

namespace CoalesceScope;

function copiedOffset(array $values): void
{
    $copy = $values['optional'];
    echo $copy;
}

function optionalWrite(?string $value): void
{
    $value ?? ($created = 1);
    echo $created;

    $existing = 2;
    $value ?? ($existing = 3);
    \PHPStan\dumpType($existing);
}

function requiredWrite(): void
{
    $null = null;
    $null ?? ($required = 1);
    echo $required;

    $absent ?? ($alsoRequired = 1);
    echo $alsoRequired;
    echo $absent;
}

function skippedWrite(): void
{
    $set = 'set';
    $set ?? ($skipped = 1);
    echo $skipped;
}

function selfWrite(bool $flag): void
{
    if ($flag) {
        $value = 'set';
    }
    $value ?? ($value = 'fallback');
    echo $value;
    \PHPStan\dumpType($value);
}

function terminatingWrite(?string $value): void
{
    $existing = 2;
    $value ?? throw new \RuntimeException((string) ($existing = 3));
    \PHPStan\dumpType($existing);
    \PHPStan\dumpType($value);
}

function lhsSideEffect(array $values): void
{
    $index = 0;
    $values[$index++] ?? throw new \RuntimeException();
    \PHPStan\dumpType($index);
}

function sourceCertaintyIsNotCopied(bool $flag): void
{
    if ($flag) {
        $source = 1;
    }
    $copy = $source;
    echo $copy;
    echo $source;
}
