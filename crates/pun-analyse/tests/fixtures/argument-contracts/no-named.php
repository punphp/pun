<?php declare(strict_types=1);

namespace ArgumentNamedCallbacks;

/** @no-named-arguments */
function blocked(int $i): int { return $i; }
/** @no-named-arguments */
function otherBlocked(int $i): int { return $i; }
function permitted(int $i): int { return $i; }

class Blocked
{
    /** @no-named-arguments */
    public function method(int $i): int { return $i; }
}

class ChildBlocked extends Blocked
{
    public function method(int $renamed): int { return $renamed; }
}

function calls(Blocked $object, ChildBlocked $child, bool $choose): void
{
    $first = blocked(...);
    $result = call_user_func($first, i: 1);
    $result = call_user_func(__NAMESPACE__.'\\blocked', i: 1);
    $result = call_user_func([$object, 'method'], i: 1);
    $result = call_user_func($object->method(...), i: 1);
    $result = call_user_func_array($first, ['i' => 1]);
    $result = $first(i: 1);
    $allowed = permitted(...);
    $result = $allowed(i: 1);
    $one = $choose ? blocked(...) : permitted(...);
    $result = call_user_func($one, i: 1);
    $both = $choose ? blocked(...) : otherBlocked(...);
    $result = $both(i: 1);
    $result = call_user_func($child->method(...), renamed: 1);
}
