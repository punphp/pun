<?php declare(strict_types=1);

namespace SequentialScopeOrder;

final class Zulu
{
    public function ping(): int { return 1; }
}

final class Alpha
{
    public function ping(): int { return 2; }
}

final class Middle
{
    public function ping(): int { return 3; }
}

function parameterOrder(Zulu|Alpha $forward, Alpha|Zulu $backward): Zulu|Alpha
{
    $forward->ping();
    $backward->ping();
    return $forward;
}

function largerLast(bool $flag, Zulu|Alpha $forward, Alpha|Zulu $backward): array
{
    if ($flag) {
        $result = ['object' => $forward, 'extra' => 1];
    } else {
        $result = ['object' => $backward];
    }
    $result['object']->ping();
    return $result;
}

function largerFirst(bool $flag, Zulu|Alpha $forward, Alpha|Zulu $backward): array
{
    if ($flag) {
        $result = ['object' => $forward];
    } else {
        $result = ['object' => $backward, 'extra' => 1];
    }
    $result['object']->ping();
    return $result;
}

function largerBeforeUnrelated(int $pick, Zulu|Alpha $forward, Alpha|Zulu $backward, Middle $middle): array
{
    if ($pick === 0) {
        $result = ['kind' => 'pair', 'object' => $forward];
    } elseif ($pick === 1) {
        $result = ['kind' => 'separate', 'object' => $middle];
    } else {
        $result = ['kind' => 'pair', 'object' => $backward, 'extra' => 1];
    }
    $result['object']->ping();
    return $result;
}

function largerAfterUnrelated(int $pick, Zulu|Alpha $forward, Alpha|Zulu $backward, Middle $middle): array
{
    if ($pick === 0) {
        $result = ['kind' => 'pair', 'object' => $forward, 'extra' => 1];
    } elseif ($pick === 1) {
        $result = ['kind' => 'separate', 'object' => $middle];
    } else {
        $result = ['kind' => 'pair', 'object' => $backward];
    }
    $result['object']->ping();
    return $result;
}

/**
 * @param array{object: Zulu|Alpha, a?: 1, b: 2} $left
 * @param array{object: Alpha|Zulu, a: 1, b?: 2} $right
 */
function equalKeys(bool $flag, array $left, array $right): array
{
    if ($flag) {
        $result = $left;
    } else {
        $result = $right;
    }
    $result['object']->ping();
    return $result;
}

function taggedAlternative(bool $flag, Zulu|Alpha $forward, Alpha|Zulu $backward): array
{
    if ($flag) {
        $result = ['kind' => 'first', 'number' => 1, 'object' => $forward];
    } else {
        $result = ['kind' => 'second', 'number' => 2, 'object' => $backward];
    }
    if ($result['kind'] === 'first') {
        $result['object']->ping();
    }
    return $result;
}
