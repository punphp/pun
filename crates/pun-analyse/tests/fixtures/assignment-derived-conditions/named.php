<?php declare(strict_types=1);

namespace AssignmentDerivedConditions;

class Target
{
    public function first(int $left): void {}
    public function second(int $right): void {}
}
class Child extends Target
{
    public function first(int $renamedLeft): void {}
    public function second(int $renamedRight): void {}
}

function named(Target $object, bool $flag): void
{
    if ($flag) {
        $method = 'first';
        $key = 'left';
    } else {
        $method = 'second';
        $key = 'right';
    }
    $arguments = [$key => 1];
    if ($method === 'first') {
        $object->first(...$arguments);
    } else {
        $object->second(...$arguments);
    }
}

function reversedNamed(Target $object, bool $flag): void
{
    if ($flag) {
        $method = 'first';
        $key = 'right';
    } else {
        $method = 'second';
        $key = 'left';
    }
    $arguments = [$key => 1];
    if ($method === 'first') {
        $object->first(...$arguments);
    } else {
        $object->second(...$arguments);
    }
}
