<?php
function mixedFlow(mixed $input, bool $choose): void
{
    if ($input === 0) {
        return;
    }
    // The test supplies the narrowed value through Analyzer's public expression hook.
    $seed = $input;
    \PHPStan\dumpType($seed);
    $copy = $seed;
    \PHPStan\dumpType($copy);
    $other = $choose ? $seed : 13;
    \PHPStan\dumpType($other);
    $nullable = $choose ? $seed : null;
    \PHPStan\dumpType($nullable);
    $filled = $choose ? $seed : 0;
    \PHPStan\dumpType($filled);
    $offset = $seed['anything'];
    \PHPStan\dumpType($offset);
    $seed = 13;
    \PHPStan\dumpType($seed);
    if ($copy !== null) {
        if ($copy !== null) {
            \PHPStan\dumpType($copy);
        }
    }
}
