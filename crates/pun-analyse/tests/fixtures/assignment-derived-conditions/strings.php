<?php declare(strict_types=1);

namespace AssignmentDerivedConditions;

class ImpureString
{
    /** @phpstan-impure */
    public function __toString(): string { echo ''; return 'text'; }
}

function strings(bool $flag, ImpureString $object): bool
{
    if ($flag) {
        $method = 'first';
        $key = 'left';
    } else {
        $method = 'second';
        $key = 'right';
    }
    $concat = [$key, 'prefix-' . $object];
    $cast = [$key, (string) $object];
    $interpolated = [$key, "{$object}"];
    $scalarConcat = 'prefix-' . $key;
    $scalarCast = (string) $key;
    $scalarInterpolation = "prefix-$key";
    if ($method === 'first') {
        \PHPStan\dumpType($concat[0]);
        \PHPStan\dumpType($cast[0]);
        \PHPStan\dumpType($interpolated[0]);
        \PHPStan\dumpType($scalarConcat);
        \PHPStan\dumpType($scalarCast);
        \PHPStan\dumpType($scalarInterpolation);
    } else {
        \PHPStan\dumpType($concat[0]);
        \PHPStan\dumpType($cast[0]);
        \PHPStan\dumpType($interpolated[0]);
        \PHPStan\dumpType($scalarConcat);
        \PHPStan\dumpType($scalarCast);
        \PHPStan\dumpType($scalarInterpolation);
    }
    return true;
}
