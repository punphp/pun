<?php

namespace ExtendedGenericBoundary;

/** @template T */
class Holder
{
    /** @param T $value */
    public function value(mixed $value, int $concrete = 0): void {}
}

/** @phpstan-require-extends Holder<int> */
interface IntRequirement {}

/** @phpstan-require-extends Holder<string> */
interface StringRequirement {}

function required(IntRequirement $ints, StringRequirement $strings): void
{
    $ints->value(1);
    $strings->value('ok');
    $ints->value('wrong');
    $strings->value(1);
}

/** @mixin Holder<int> */
class Proxy {}

/** @mixin Proxy */
class NestedProxy {}

/** @phpstan-require-extends Proxy */
interface IndirectRequirement {}

function extended(Proxy $proxy, NestedProxy $nested, IndirectRequirement $required): void
{
    $proxy->value(1);
    $nested->value(1);
    $required->value(1);
    $proxy->value('wrong');
    $nested->value('wrong');
    $required->value('wrong');
}

/**
 * @param Holder<int> $ints
 * @param Holder<string> $strings
 */
function ordinary(Holder $ints, Holder $strings): void
{
    $ints->value(1);
    $strings->value('ok');
    $ints->value('wrong');
    $strings->value(1);
    // Concrete native parameters must remain checked too.
    $ints->value(1, 'wrong');
    $strings->value('ok', 'wrong');
}

class PolicyAllowed
{
    public function run(string $old): void {}
}

class PolicyRenamed extends PolicyAllowed
{
    public function run(string $new): void {}
}

/** @no-named-arguments */
class PolicyDenied
{
    public function run(string $old): void {}
}

/**
 * @template T of object
 * @mixin T
 */
class PolicyProxy {}

/** @param PolicyProxy<PolicyAllowed>|PolicyProxy<PolicyDenied> $object */
function combinedNamedPolicy(PolicyProxy $object): void
{
    $object->run(old: 'combined');
}

/** @param PolicyProxy<PolicyAllowed> $object */
function allowedNamedPolicy(PolicyProxy $object): void
{
    $object->run(old: 'allowed');
}

/** @param PolicyProxy<PolicyDenied> $object */
function deniedNamedPolicy(PolicyProxy $object): void
{
    $object->run(old: 'denied');
}

/** @param PolicyProxy<PolicyAllowed>|PolicyProxy<PolicyDenied>|null $object */
function nullableNamedPolicy(?PolicyProxy $object): void
{
    $object?->run(old: 'nullable combined');
}
