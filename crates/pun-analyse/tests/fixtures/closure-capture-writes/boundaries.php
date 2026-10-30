<?php declare(strict_types=1);

namespace ClosureCaptureBoundaries;

interface Driver
{
    /** @throws \LogicException */
    public function run(): void;
}

final class Guard
{
    public function __destruct() { echo 'released'; }
}

function boundaries(int $value, bool $flag, Driver $driver, Guard $guard): void
{
    $skippedCatch = function () use ($value, $driver): void {
        try {
            $driver->run();
        } catch (\RuntimeException $e) {
            echo $value;
        }
    };
    $matchingCatch = function () use ($value, $driver): void {
        try {
            $driver->run();
        } catch (\LogicException $e) {
            echo $value;
        }
    };
    $overwrittenBeforeThrow = function () use ($value, $driver): void {
        try {
            $value = 10;
            $driver->run();
        } catch (\LogicException $e) {
            echo $value;
        }
    };
    $throwBeforeWrite = function () use ($value, $driver): void {
        try {
            $driver->run();
            $value = 10;
        } catch (\LogicException $e) {
            echo $value;
        }
    };
    $finallyOverwrites = function () use ($value): int {
        try {
            $value = 10;
        } finally {
            $value = 20;
        }
        return $value;
    };
    $finallyReads = function () use ($value): int {
        try {
            return 10;
        } finally {
            echo $value;
        }
    };
    $destructor = function () use ($guard): void {
        $guard = new Guard();
    };
    $alias = function () use ($value): int {
        $alias =& $value;
        $value = 10;
        return $alias;
    };
    $doWhile = function () use ($value, $flag): int {
        do {
            $value = 10;
        } while ($flag);
        return $value;
    };
    $while = function () use ($value, $flag): int {
        while ($flag) {
            $value = 10;
        }
        return $value;
    };
    $whileRead = function () use ($value, $flag): void {
        while ($flag) {
            echo $value;
            $value = 10;
        }
    };
    $stop = function () use ($value): int {
        return 10;
        echo $value;
    };
}
