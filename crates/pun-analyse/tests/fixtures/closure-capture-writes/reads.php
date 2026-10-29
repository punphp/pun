<?php declare(strict_types=1);

namespace ClosureCaptureReads;

function reads(int $value, bool $flag, string $name): void
{
    $readBeforeWrite = function () use ($value): void {
        echo $value;
        $value = 10;
    };
    $compound = function () use ($value): int {
        $value += 10;
        return $value;
    };
    $increment = function () use ($value): int {
        return ++$value;
    };
    $rhs = function () use ($value): int {
        $value = $value + 10;
        return $value;
    };
    $copy = function () use ($value): void {
        $deadCopy = $value;
    };
    $oneBranch = function () use ($value, $flag): int {
        if ($flag) {
            $value = 10;
        }
        return $value;
    };
    $referenceWrite = function () use (&$value): void { $value = 10; };
    $referenceUnset = function () use (&$value): void { unset($value); };
    $nestedReference = function () use ($value): \Closure {
        return function () use (&$value): void { $value = 10; };
    };
    $arrowBeforeWrite = function () use ($value): \Closure {
        $arrow = fn (): int => $value;
        $value = 10;
        return $arrow;
    };
    $dynamic = function () use ($value, $name): void {
        echo $$name;
    };
    $dynamicAfterWrite = function () use ($value, $name): void {
        $value = 10;
        echo $$name;
    };
    $compact = function () use ($value): array {
        return compact('value');
    };
    $nestedCompact = function () use ($value): array {
        return compact(['value']);
    };
    $defined = function () use ($value): array {
        return get_defined_vars();
    };
    $definedAfterWrite = function () use ($value): array {
        $value = 10;
        return get_defined_vars();
    };
    $argumentNames = function () use ($value): array {
        $value = 10;
        return func_get_args();
    };
    $evaluate = function () use ($value): void {
        eval('echo $value;');
    };
    $opaque = function () use ($value): void {
        goto end;
        end: echo 'done';
    };
    $argumentName = function () use ($value): mixed {
        return func_get_arg(0);
    };
    $dynamicCompact = function () use ($value, $name): array {
        return compact($name);
    };
    $extract = function () use ($value): void {
        extract(['value' => 10]);
    };
    $referenceUnreachable = function () use (&$value): int {
        return 10;
        echo $value;
    };
}
