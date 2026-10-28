<?php declare(strict_types=1);

namespace FilteredNamedArguments;

class Target
{
    public function first(mixed $before = null, int $left = 0, int $right = 0): void {}
    public function second(mixed $before = null, int $left = 0, int $right = 0): void {}
}
class Child extends Target
{
    public function first(mixed $before = null, int $renamedLeft = 0, int $right = 0): void {}
    public function second(mixed $before = null, int $left = 0, int $renamedRight = 0): void {}
}

function derivedLiteralKey(bool $flag, Target $object): void
{
    if ($flag) {
        $method = 'first';
        $key = 'left';
    } else {
        $method = 'second';
        $key = 'right';
    }
    $object->$method(...[$key => 1]);
}

function asymmetricLiteralKey(bool $flag, Target $object): void
{
    if ($flag) {
        $method = 'first';
        $key = 'right';
    } else {
        $method = 'second';
        $key = 'left';
    }
    $object->$method(...[$key => 1]);
}

function stableOffsetSelector(bool $flag, Target $object): void
{
    $pair = $flag
        ? ['method' => 'first', 'key' => 'left', 'arguments' => ['left' => 1]]
        : ['method' => 'second', 'key' => 'right', 'arguments' => ['right' => 1]];
    $object->{$pair['method']}(...$pair['arguments']);
    $object->{$pair['method']}(...[$pair['key'] => 1]);
}

function ternarySelector(bool $flag, Target $object): void
{
    if ($flag) {
        $arguments = ['left' => 1];
    } else {
        $arguments = ['right' => 1];
    }
    $object->{($flag ? 'first' : 'second')}(...$arguments);
}

function singleConstantAndIdentifier(Target $object): void
{
    $method = 'first';
    $arguments = ['left' => 1];
    $object->$method(...$arguments);
    $object->first(...$arguments);
    $object->{'first'}(...$arguments);
}

function earlierWrite(Target $object): void
{
    $method = 'first';
    $arguments = ['right' => 1];
    $object->$method($arguments = ['left' => 1], ...$arguments);
    $arguments = ['right' => 1];
    $object->first($arguments = ['left' => 1], ...$arguments);
}

function laterWrite(Target $object): void
{
    $method = 'first';
    $arguments = ['left' => 1];
    $object->$method(...$arguments, before: $arguments = ['right' => 1]);
    $arguments = ['left' => 1];
    $object->first(...$arguments, before: $arguments = ['right' => 1]);
}

function correlatedWrite(bool $flag, Target $object): void
{
    if ($flag) {
        $method = 'first';
        $key = 'left';
    } else {
        $method = 'second';
        $key = 'right';
    }
    $arguments = [];
    $object->$method($arguments = [$key => 1], ...$arguments);
}

class ArgumentBox
{
    public array $arguments = [];

    /** @phpstan-impure */
    public function mutate(): int
    {
        $this->arguments = [];
        return 1;
    }

    /** @phpstan-pure */
    public function pure(): int
    {
        return 1;
    }
}

function earlierImpurity(Target $object, ArgumentBox $box): void
{
    $method = 'first';
    $box->arguments = ['left' => 1];
    $object->$method($box->mutate(), ...$box->arguments);
    $box->arguments = ['left' => 1];
    $object->$method($box->pure(), ...$box->arguments);
    $box->arguments = ['left' => 1];
    $object->first($box->mutate(), ...$box->arguments);
    $box->arguments = ['left' => 1];
    $object->first($box->pure(), ...$box->arguments);
}

function laterImpurity(Target $object, ArgumentBox $box): void
{
    $method = 'first';
    $box->arguments = ['left' => 1];
    $object->$method(...$box->arguments, before: $box->mutate());
    $box->arguments = ['left' => 1];
    $object->first(...$box->arguments, before: $box->mutate());
}
