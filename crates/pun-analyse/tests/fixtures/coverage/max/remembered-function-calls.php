<?php declare(strict_types=1);

namespace MaxRememberedCalls;

function value(string $key): mixed
{
    return $key;
}

/** @phpstan-impure */
function impureValue(string $key): mixed
{
    echo $key;

    return $key;
}

// rememberPossiblyImpureFunctionValues: a repeated call with the same arguments keeps its narrowed type
function confirmedAt(): int
{
    $at = is_int(value('auth.confirmed_at')) ? value('auth.confirmed_at') : 0;
    \PHPStan\dumpType($at);
    if (is_int(value('a'))) {
        \PHPStan\dumpType(value('a'));
        \PHPStan\dumpType(value('b'));
    }
    if (is_int(impureValue('a'))) {
        \PHPStan\dumpType(impureValue('a'));
    }

    return time() - $at;
}
