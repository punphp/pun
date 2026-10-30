<?php declare(strict_types=1);

namespace AssignmentDerivedConditions;

class RetainedTarget
{
    public function first(int $left): void {}
    public function second(int $right): void {}
}

function retained(bool $flag): bool
{
    if ($flag) {
        $method = 'first';
        $key = 'left';
    } else {
        $method = 'second';
        $key = 'right';
    }
    $arguments = [$key => 1];
    $object = new RetainedTarget;
    $object->$method(...$arguments);
    if ($method === 'first') {
        \PHPStan\dumpType($arguments);
    } else {
        \PHPStan\dumpType($arguments);
    }
    return true;
}
