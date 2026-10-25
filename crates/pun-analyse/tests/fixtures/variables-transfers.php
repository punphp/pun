<?php

namespace VariableTransfers;

function annotations(bool $flag, array $input): void
{
    /** @var string $absent */
    echo $absent;
    compact('absent');
    if ($flag) {
        $maybe = 1;
    }
    /** @var string $maybe */
    echo $maybe;
    compact('maybe');
    extract($input);
    echo $extracted;
    compact('extracted');
    /** @var string $declared */
    echo $declared;
    compact('declared');
}

function included(bool $flag): void
{
    if ($flag) {
        include 'unknown.php';
    }
    echo $conditional;
    compact('conditional');
    include_once 'unknown.php';
    /** @var string $declared */
    echo $declared;
    compact('declared');
    echo $unknown;
    compact('unknown');
}

function correlated(bool $flag): void
{
    if ($flag) {
        $name = 'left';
        $left = 1;
    } else {
        $name = 'right';
        $right = 1;
    }
    echo $$name;
    if ($name === 'left') {
        $right = 2;
        echo $right;
    }
    $name = 'left';
    echo $left;
    compact('left');
}

function caught(bool $flag): void
{
    try {
        if ($flag) {
            $one = 1;
            throw new \LogicException();
        }
        $two = 2;
        throw new \RuntimeException();
    } catch (\Exception $caught) {
        echo $one, $two;
        compact('one', 'two', 'caught');
    }
    try {
        try {
            $nested = 1;
            throw new \RuntimeException();
        } catch (\LogicException $notCaught) {
            echo $nested;
        }
    } catch (\Exception $caught) {
        echo $nested;
        compact('nested');
    }
}

function branching(): void
{
    if (false) {
        $never = 1;
    } elseif (false) {
        $alsoNever = 1;
    } elseif (true) {
        $always = 1;
    } else {
        $otherwise = 1;
    }
    echo $never, $alsoNever, $always, $otherwise;
    compact('never', 'alsoNever', 'always', 'otherwise');
    for ($i = 0; $i < 2; ++$i) {
        if (false) {
            continue;
        }
        $fallthrough = 1;
    }
    echo $fallthrough;
    compact('fallthrough');
}

function maybeLoop(bool $flag): void
{
    for ($i = 0; $i < 2; ++$i) {
        if ($flag) {
            continue;
        }
        $fallthrough = 1;
    }
    echo $fallthrough;
    compact('fallthrough');
}

function absentGuard(): void
{
    if (isset($missing)) {
        echo $missing;
    }
    $missing = 1;
    echo $missing;
    compact('missing');
}

/** @throws \LogicException */
function documented(): void {}
/** @throws void */
function noThrow(): void {}
function undocumented(): void {}

function throwTransfers(callable $unknown): void
{
    try {
        $before = 1;
        documented();
    } catch (\LogicException $caught) {
        echo $before;
        compact('before', 'caught');
    }
    try {
        $unknown();
        $after = 1;
    } catch (\Exception $caught) {
        echo $after;
        compact('after');
    }
    try {
        undocumented();
        $afterCall = 1;
        throw new \LogicException();
    } catch (\LogicException $caught) {
        echo $afterCall;
        compact('afterCall');
    }
    try {
        undocumented();
        $beforeDocumented = 1;
        documented();
    } catch (\LogicException $caught) {
        echo $beforeDocumented;
        compact('beforeDocumented');
    }
    try {
        noThrow();
        $beforeNoThrow = 1;
        throw new \LogicException();
    } catch (\LogicException $caught) {
        echo $beforeNoThrow;
        compact('beforeNoThrow');
    }
}

function mismatchedName(bool $flag): void
{
    if ($flag) {
        $name = 'right';
        $left = 1;
    } else {
        $name = 'left';
        $right = 1;
    }
    echo $$name;
    echo ${$name};
}

/** @param callable-string $callableName
 * @param numeric-string $numericName
 * @param array-key $key
 */
function nonLiteralNames(string $callableName, string $numericName, int|string $key): void
{
    echo $$callableName, $$numericName;
    compact($callableName, $numericName, $key);
}

function forgottenGuard(bool $flag): void
{
    if ($flag) {
        $tag = 'left';
        $left = 1;
    } else {
        $tag = 'right';
        $right = 1;
    }
    unset($tag);
    if (isset($left)) {
        compact('left');
    }
    echo $right;
    compact('right');
}

function builtinReferences(\mysqli_stmt $statement, bool $flag): void
{
    $statement->bind_result($output);
    echo $output;
    $statement->bind_param('s', $input);
    echo $input;
    \Uri\WhatWg\Url::parse('https://example.com', null, $errors);
    echo $errors;
    new \Uri\WhatWg\Url('https://example.com', null, $constructorErrors);
    echo $constructorErrors;
    $statement->bind_param('ss', $later, $later = 'assigned');
    echo $later;
    if ($flag) {
        $maybeInput = 1;
    }
    $statement->bind_param('s', $maybeInput);
    echo $maybeInput;
    $existing = [2, 1];
    sort($existing);
    echo $existing;
}

function flag(): bool
{
    return (bool) rand();
}

function opaqueCondition(): void
{
    if (flag()) {
        $conditional = 1;
    }
    echo $conditional;
    compact('conditional');
    try {
        rand();
        $assigned = 1;
        throw new \LogicException();
    } catch (\LogicException $caught) {
        echo $assigned;
        compact('assigned');
    }
}

function falseWithoutElse(): void
{
    if (false) {
        $never = 1;
    }
    echo $never;
    compact('never');
}

// Invalid generator return types leave Mago's yield expression untyped. No constant name is known.
function unknownYieldName(): void
{
    echo ${yield};
    compact(yield);
}

function unreachableKnown(): void
{
    $known = 1;
    return;
    compact('known');
}

function closureReferences(): void
{
    $write = function (int &$output): void {};
    $write($positional);
    $write(output: $named);
    echo $positional, $named;
    $variadic = function (int &...$outputs): void {};
    $variadic($first, $second);
    $variadic(other: $namedVariadic);
    echo $first, $second, $namedVariadic;
    $arrow = fn (int &$output) => null;
    $arrow($arrowOutput);
    echo $arrowOutput;
    $read = function (int $input): void {};
    $read($missingInput);
    nativeReference($missingNativeInput);
}

function nativeReference(int &$output): void {}

/** @param callable(int &$output): void $write */
function typedCallableReference(callable $write): void
{
    $write($typedOutput);
    echo $typedOutput;
}
