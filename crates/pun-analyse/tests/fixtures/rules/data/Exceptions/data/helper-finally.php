<?php

namespace HelperFinally;

function conditional(bool $flag): int
{
    try { return 1; } finally {
        if ($flag) return 2;
    }
}

function exhaustive(bool $flag): int
{
    try { return 1; } finally {
        if ($flag):
            return 2;
        elseif (false):
            return 3;
        else:
            throw new \Exception();
        endif;
    }
}

function literalBranch(): int
{
    try { return 1; } finally {
        if (false) { echo 'skip'; }
        elseif (true) { return 2; }
        else { echo 'skip'; }
    }
}

function switchAll(int $n): int
{
    try { return 1; } finally {
        switch ($n) {
            case 1:
            case 2: return 2;
            default: throw new \Exception();
        }
    }
}

function switchBreak(int $n): int
{
    try { return 1; } finally {
        switch ($n) {
            case 1: break;
            default: return 2;
        }
    }
}

function switchNoDefault(int $n): int
{
    try { return 1; } finally {
        switch ($n) { case 1: return 2; }
    }
}

function whileReturn(): int
{
    try { return 1; } finally {
        while (true) { return 2; }
    }
}

function whileBreak(bool $flag): int
{
    try { return 1; } finally {
        while (true) { if ($flag) break; return 2; }
    }
}

function whileFalse(): int
{
    try { while (false) { return 1; } return 2; }
    finally { return 3; }
}

function forReturn(): int
{
    try { return 1; } finally {
        for (;;) { return 2; }
    }
}

function forFalse(): int
{
    try { return 1; } finally {
        for (; false;) { return 2; }
    }
}

function doReturn(): int
{
    try { return 1; } finally {
        do { return 2; } while (false);
    }
}

function doContinue(bool $flag): int
{
    try { return 1; } finally {
        do { if ($flag) continue; return 2; } while (false);
    }
}

function foreachReturn(array $items): int
{
    try { foreach ($items as $item) { if ($item) return 1; } return 2; }
    finally { return 3; }
}

function jumps(bool $flag): void
{
    while ($flag) {
        try { if (random_int(0, 1)) continue; else break; }
        finally { return; }
    }
}

function overwritingBreak(bool $flag): void
{
    while ($flag) {
        try { return; } finally { break; }
    }
}

function overwritingContinue(bool $flag): void
{
    while ($flag) {
        try { return; } finally { continue; }
    }
}

function multiLevel(bool $flag): int
{
    while ($flag) {
        try { while ($flag) { break 2; } return 1; }
        finally { return 2; }
    }
    return 3;
}

function noFinally(): void
{
    try { echo 'try'; } catch (\Throwable $e) { echo 'catch'; }
}

function scopes(): int
{
    try { $cb = fn () => 1; function inner(): int { return 2; } }
    finally { return 3; }
}

function tryExit(): int
{
    try { exit; } finally { return 1; }
}

function finallyExit(): int
{
    try { return 1; } finally { exit; }
}

function falsyVariable(): int
{
    $flag = 0;
    try { return 1; } finally {
        if ($flag) return 2;
        return 3;
    }
}
