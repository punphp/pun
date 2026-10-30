<?php declare(strict_types=1);

namespace SequentialConstraintOrder;

final class Zulu
{
    public function ping(): int { return 1; }
}

final class Alpha
{
    public function ping(): int { return 2; }
}

/**
 * @template T of array{kind: 'first', tag: 1, object: Zulu}|array{kind: 'second', tag: 2, object: Alpha}
 * @param T $result
 * @return T
 */
function forward(array $result): array
{
    $result['object']->ping();
    return $result;
}

/**
 * @template T of array{kind: 'second', tag: 2, object: Alpha}|array{kind: 'first', tag: 1, object: Zulu}
 * @param T $result
 * @return T
 */
function backward(array $result): array
{
    $result['object']->ping();
    return $result;
}
