<?php declare(strict_types=1);

namespace ClosureCaptureWrites;

function overwrites(int $left, int $right, bool $flag): void
{
    $plain = function () use ($left): int {
        $left = 10;
        return $left;
    };
    $asymmetric = function () use ($left, $right): int {
        $left = $right;
        return $left;
    };
    $readOnRight = function () use ($left, $right): int {
        $right = $left;
        return $right;
    };
    $destructure = function () use ($left, $right): int {
        [$left, $right] = [10, 20];
        return $left + $right;
    };
    $bothBranches = function () use ($left, $flag): int {
        if ($flag) {
            $left = 10;
        } else {
            $left = 20;
        }
        return $left;
    };
    $oneBranch = function () use ($left, $flag): int {
        if ($flag) {
            $left = 10;
        }
        return $left;
    };
    $readOnlyAfterJoin = function () use ($left, $right, $flag): int {
        if ($flag) {
            $left = $right;
        } else {
            $left = 20;
        }
        return $left;
    };
    $readBeforeJoin = function () use ($left, $right, $flag): int {
        if ($flag) {
            $right = $left;
        } else {
            $left = 20;
        }
        return $left + $right;
    };
    $nestedAfterWrite = function () use ($left): \Closure {
        $left = 10;
        return function () use ($left): int { return $left; };
    };
    $nestedBeforeWrite = function () use ($left): \Closure {
        $nested = function () use ($left): int { return $left; };
        $left = 10;
        return $nested;
    };
    $arrowAfterWrite = function () use ($left): \Closure {
        $left = 10;
        return fn (): int => $left;
    };
    $arrowBeforeWrite = function () use ($left): \Closure {
        $nested = fn (): int => $left;
        $left = 10;
        return $nested;
    };
    $arrowOwnWrite = function () use ($left): \Closure {
        return fn (): int => $left = 10;
    };
    $arrowShadows = function () use ($left): \Closure {
        return fn (int $left): int => $left;
    };
    $arrowDoesNotWriteOutside = function () use ($left): int {
        $nested = fn (): int => $left = 10;
        return $left;
    };
    $earlyExit = function () use ($left, $flag): int {
        if ($flag) {
            return 10;
        }
        $left = 20;
        return $left;
    };
    $conditionalWrite = function () use ($left, $flag): int {
        $flag ? $left = 10 : $left = 20;
        return $left;
    };
    $shortCircuit = function () use ($left, $flag): int {
        $flag && $left = 10;
        return $left;
    };
    $unusedByReference = function () use (&$right): void {};
    $writeByReference = function () use (&$left): void { $left = 10; };
    $unusedValue = function () use ($left): void {};
}
