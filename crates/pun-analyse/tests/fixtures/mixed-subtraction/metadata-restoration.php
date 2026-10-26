<?php

namespace MixedMetadataRestoration;

/** @param array{inner: array} $shape */
function inferredExplicit(array $shape): void
{
    \PHPStan\dumpType($shape);
    $copy = $shape;
    if (is_array($copy['inner'])) {
        \PHPStan\dumpType($copy);
        \PHPStan\dumpType($shape);
        $alias = $copy;
        \PHPStan\dumpType($alias);
        $alias['inner'] = ['fixed' => 1];
        \PHPStan\dumpType($alias);
    }
}

/** @param array{inner: array<mixed, mixed>} $shape */
function declaredExplicit(array $shape): void
{
    \PHPStan\dumpType($shape);
    $copy = $shape;
    \PHPStan\dumpType($copy);
    if (is_array($copy['inner'])) {
        \PHPStan\dumpType($copy);
    }
    $copy['inner'] = ['fixed' => 2];
    \PHPStan\dumpType($copy);
}
