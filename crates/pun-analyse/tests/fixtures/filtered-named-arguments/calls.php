<?php declare(strict_types=1);

namespace DynamicNamedScope;

class ALeft
{
    public function first(int $value): void {}
    public function second(int $value): void {}
}
class ALeftChild extends ALeft
{
    public function first(int $renamed): void {}
}
class BRight
{
    public function first(int $value): void {}
    public function second(int $value): void {}
}
class BRightChild extends BRight
{
    public function second(int $renamed): void {}
}

function correlatedReceiver(bool $flag, ALeft $left, BRight $right): void
{
    if ($flag) {
        $method = 'first';
        $object = $left;
    } else {
        $method = 'second';
        $object = $right;
    }
    $object->$method(value: 1);
    $left->second(value: 1);
    $right->first(value: 1);
}

function asymmetricReceiver(bool $flag, ALeft $left, BRight $right): void
{
    if ($flag) {
        $method = 'second';
        $object = $left;
    } else {
        $method = 'first';
        $object = $right;
    }
    $object->$method(value: 1);
}

class UnpackTarget
{
    public function first(int $left = 0, int $right = 0): void {}
    public function second(int $left = 0, int $right = 0): void {}
}
class UnpackChild extends UnpackTarget
{
    public function first(int $renamedLeft = 0, int $right = 0): void {}
    public function second(int $left = 0, int $renamedRight = 0): void {}
}

function correlatedUnpack(bool $flag, UnpackTarget $object): void
{
    if ($flag) {
        $method = 'first';
        $arguments = ['left' => 1];
    } else {
        $method = 'second';
        $arguments = ['right' => 1];
    }
    $object->$method(...$arguments);
}

function asymmetricUnpack(bool $flag, UnpackTarget $object): void
{
    if ($flag) {
        $method = 'first';
        $arguments = ['right' => 1];
    } else {
        $method = 'second';
        $arguments = ['left' => 1];
    }
    $object->$method(...$arguments);
}

function nullableReceiver(bool $flag, ?ALeft $left, ?BRight $right): void
{
    if ($flag) {
        $method = 'first';
        $object = $left;
    } else {
        $method = 'second';
        $object = $right;
    }
    $object?->$method(value: 1);
}

function actualBranch(bool $flag, ALeft $left, BRight $right): void
{
    if ($flag) {
        $method = 'first';
        $object = $left;
    } else {
        $method = 'second';
        $object = $right;
    }
    if ($method === 'second') {
        $object->second(value: 1);
    }
}

function literalNames(ALeft $left, BRight $right, UnpackTarget $unpack): void
{
    $left->first(value: 1);
    $right->second(value: 1);
    $left->{'first'}(value: 1);
    $unpack->first(...['left' => 1]);
    $unpack->second(...['right' => 1]);
}

function unrelatedBranches(bool $names, bool $objects, ALeft $left, BRight $right): void
{
    if ($names) {
        $method = 'first';
    } else {
        $method = 'second';
    }
    if ($objects) {
        $object = $left;
    } else {
        $object = $right;
    }
    $object->$method(value: 1);
}

function selectorWriteInvalidates(bool $flag, ALeft $left, BRight $right): void
{
    if ($flag) {
        $method = 'first';
        $object = $left;
    } else {
        $method = 'second';
        $object = $right;
    }
    $method = 'first';
    $object->$method(value: 1);
}
