<?php
declare(strict_types=1);

namespace ArrayMapAcceptorShapes;

/**
 * @template T
 * @param T $value
 * @return T
 */
function identity(mixed $value): mixed { return $value; }

/**
 * @param array{head?: int, ...<string, int>} $openShape
 * @param list{int, ...<int>} $openList
 */
function probe(array $openShape, array $openList): void
{
    \PHPStan\dumpType(array_map(identity(...), ['head' => 1]));
    \PHPStan\dumpType(array_map(identity(...), $openShape));
    \PHPStan\dumpType(array_map(identity(...), $openList));
}
