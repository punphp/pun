<?php

namespace TryCertainty;

class NeverThrows
{
    /** @throws void */
    final public function __construct() {}
}

class DeclaredThrows
{
    /** @throws \ArgumentCountError */
    public function __construct() {}
}

class ImplicitThrows
{
    public function __construct() {}
}

final class NoConstructor {}
class ExtensibleNoConstructor {}

/** @throws void */
function neverThrows(): void {}

/** @throws \ArgumentCountError */
function declaredThrows(): void {}

function implicitThrows(): void {}

/** @param class-string<NeverThrows> $class */
function impossibleConstructorCatch(string $class): object
{
    try {
        $object = new $class();
    } catch (\ArgumentCountError $error) {
        echo $unreachable;
    }
    return $object;
}

/** @param class-string<DeclaredThrows> $class */
function declaredConstructorCatch(string $class): object
{
    try {
        $object = new $class();
    } catch (\ArgumentCountError $error) {
    }
    return $object;
}

/** @param class-string<ImplicitThrows> $class */
function implicitConstructorCatch(string $class): object
{
    try {
        $object = new $class();
    } catch (\ArgumentCountError $error) {
    }
    return $object;
}

/** @param class-string $class */
function unknownConstructorCatch(string $class): object
{
    try {
        $object = new $class();
    } catch (\ArgumentCountError $error) {
    }
    return $object;
}

/** @param class-string<NoConstructor> $class */
function finalWithoutConstructor(string $class): object
{
    try {
        $object = new $class();
    } catch (\Throwable $error) {
        echo $unreachable;
    }
    return $object;
}

/** @param class-string<ExtensibleNoConstructor> $class */
function extensibleWithoutConstructor(string $class): object
{
    try {
        $object = new $class();
    } catch (\ArgumentCountError $error) {
    }
    return $object;
}

function exactWithoutConstructor(): object
{
    try {
        $object = new ExtensibleNoConstructor();
    } catch (\ArgumentCountError $error) {
        echo $unreachable;
    }
    return $object;
}

/** @param class-string<DeclaredThrows> $class */
function terminatingCatch(string $class): object
{
    try {
        $object = new $class();
    } catch (\ArgumentCountError $error) {
        return new NeverThrows();
    }
    return $object;
}

/** @param class-string<DeclaredThrows> $class */
function precedingAssignment(string $class): object
{
    $object = new NeverThrows();
    try {
        $object = new $class();
    } catch (\ArgumentCountError $error) {
        echo $object::class;
    }
    return $object;
}

function impossibleFunctionCatch(): int
{
    try {
        neverThrows();
        $value = 1;
    } catch (\ArgumentCountError $error) {
        echo $unreachable;
    }
    return $value;
}

function impossibleAssignmentCatch(): int
{
    try {
        $value = 1;
    } catch (\ArgumentCountError $error) {
        echo $unreachable;
    }
    return $value;
}

function declaredCatchIgnoresEarlierImplicit(): int
{
    try {
        implicitThrows();
        $value = 1;
        declaredThrows();
    } catch (\ArgumentCountError $error) {
        echo $value;
    }
    return $value;
}

function expressionThrowKeepsEarlierImplicit(): int
{
    try {
        implicitThrows();
        $value = 1;
        throw new \ArgumentCountError();
    } catch (\ArgumentCountError $error) {
        echo $value;
    }
    return $value;
}

function broadCatchKeepsEarlierImplicit(): int
{
    try {
        implicitThrows();
        $value = 1;
        declaredThrows();
    } catch (\Error $error) {
        echo $value;
    }
    return $value;
}

/** @param class-string<NeverThrows> $class */
function nestedTryFinally(string $class): object
{
    try {
        try {
            $object = new $class();
        } catch (\ArgumentCountError $error) {
            echo $unreachable;
        } finally {
            echo $object::class;
            $finished = true;
        }
    } catch (\ArgumentCountError $error) {
        echo $unreachable;
    }
    echo $finished;
    return $object;
}

function nestedReachableCatchFinally(): int
{
    $value = 0;
    try {
        try {
            $value = 1;
            declaredThrows();
        } catch (\ArgumentCountError $error) {
            echo $value;
        } finally {
            $finished = true;
        }
    } catch (\LogicException $error) {
        echo $unreachable;
    }
    echo $finished;
    return $value;
}

class InstanceOnly
{
    public function instance(): void {}
}

function staticCallToInstanceMethod(): void
{
    try {
        InstanceOnly::instance();
    } catch (\Exception $exception) {
        $caught = true;
    }
    echo $caught;
}

function staticCallToUnknownMethod(): void
{
    try {
        InstanceOnly::unknown();
    } catch (\Exception $exception) {
        $caught = true;
    }
    echo $caught;
}
