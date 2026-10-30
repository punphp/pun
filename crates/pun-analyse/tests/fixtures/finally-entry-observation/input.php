<?php

namespace FinallyEntryObservation;

function stop(): never { exit; }

function impossibleNestedCatch(): void
{
    $value = 7;
    try {
        try {
            $value = 8;
        } catch (\RuntimeException $error) {
            $value = 'dead';
            return;
        }
        $value; // observe:after-inner
    } finally {
        $value; // observe:shared-finally
    }
    $value; // observe:after-shared-finally
}

function matchedThrow(): void
{
    $value = 3;
    try {
        $value = 5;
        throw new \LogicException();
    } catch (\LogicException $error) {
        $value; // observe:matching-catch
    } finally {
        $value; // observe:matching-finally
    }
    $value; // observe:matching-tail
}

function conditionalReturn(bool $pick): void
{
    $value = 11;
    try {
        if ($pick) {
            $value = 12;
            return;
        }
        $value = 13;
    } finally {
        $value; // observe:return-finally
    }
    $value; // observe:return-tail
}

function nestedReturnFinally(): void
{
    $value = 21;
    try {
        try {
            $value = 22;
            return;
        } finally {
            $value; // observe:inner-return-finally
            $value = 23;
        }
    } finally {
        $value; // observe:outer-return-finally
    }
}

function returnOperand(): int
{
    $value = 31;
    try {
        return $value = 32;
    } finally {
        $value; // observe:return-operand-finally
    }
}

function innerBreak(): void
{
    $value = 41;
    try {
        do {
            $value = 42;
            break;
        } while (false);
        $value = 43;
    } finally {
        $value; // observe:inner-break-finally
    }
}

function outerBreak(): void
{
    $value = 44;
    do {
        try {
            $value = 45;
            break;
        } finally {
            $value; // observe:outer-break-finally
            $value = 46;
        }
    } while (false);
    $value; // observe:outer-break-tail
}

function outerContinue(): void
{
    $value = 47;
    do {
        try {
            $value = 48;
            continue;
        } finally {
            $value; // observe:outer-continue-finally
            $value = 49;
        }
    } while (false);
    $value; // observe:outer-continue-tail
}

function terminatingCall(bool $pick): void
{
    $value = 50;
    try {
        if ($pick) {
            $value = 51;
            stop();
        }
        $value = 52;
    } finally {
        $value; // observe:never-finally
    }
    $value; // observe:never-tail
}

function caughtReturn(bool $pick): void
{
    $value = 60;
    try {
        if ($pick) {
            throw new \RuntimeException();
        }
        $value = 61;
    } catch (\RuntimeException $error) {
        $value = 62;
        return;
    } finally {
        $value; // observe:catch-return-finally
    }
    $value; // observe:catch-return-tail
}

function sameHolderFinally(bool $pick): void
{
    $value = 70;
    try {
        if ($pick) {
            $value = 71;
            return;
        }
        $value = 72;
    } finally {
        $value; // observe:unchanged-holder-entry
        $value = $pick ? 71 : 72;
    }
    $value; // observe:unchanged-holder-tail
}

function nestedNoFinally(bool $pick): void
{
    $value = 80;
    try {
        try {
            if ($pick) {
                $value = 81;
                return;
            }
            $value = 82;
        } catch (\RuntimeException $error) {
            $value = 83;
        }
    } finally {
        $value; // observe:nested-no-finally
    }
}

function switchBreak(bool $pick): void
{
    $value = 90;
    switch ($pick) {
        case true:
            try {
                $value = 91;
                break;
            } finally {
                $value = 92;
            }
        default:
            $value = 93;
    }
    $value; // observe:switch-break-tail
}

function switchContinue(bool $pick): void
{
    $value = 94;
    switch ($pick) {
        case true:
            try {
                $value = 95;
                continue;
            } finally {
                $value = 96;
            }
        default:
            $value = 97;
    }
    $value; // observe:switch-continue-tail
}

function finallyOverridesReturn(): void
{
    $value = 100;
    try {
        try {
            $value = 101;
            return;
        } finally {
            $value = 102;
            return;
        }
    } finally {
        $value; // observe:overridden-return-finally
    }
}

function newLocalInFinally(bool $pick): void
{
    try {
        if ($pick) {
            return;
        }
    } finally {
        $created = 103;
    }
    $created; // observe:new-finally-local
}

function mixedBreaks(bool $pick): void
{
    $value = 104;
    do {
        if ($pick) {
            $value = 105;
            break;
        }
        try {
            $value = 106;
            break;
        } finally {
            $value = 107;
        }
    } while (false);
    $value; // observe:mixed-break-tail
}

function foreachFinally(): void
{
    $value = 108;
    foreach ([1] as $item) {
        try {
            $value = 109;
            continue;
        } finally {
            $value = 110;
        }
    }
    $value; // observe:foreach-continue-tail
}

function normalAndBreak(bool $pick): void
{
    $value = 111;
    do {
        try {
            if ($pick) {
                $value = 112;
                break;
            }
            $value = 113;
        } finally {
            $created = 114;
        }
    } while (false);
    $value; // observe:normal-and-break-tail
    $created; // observe:normal-and-break-new-local
}

function optionalLoop(array $items): void
{
    $value = 120;
    foreach ($items as $item) {
        try {
            $value = 121;
            break;
        } finally {
            $value = 122;
        }
    }
    $value; // observe:optional-loop-tail
}

function breakBeforeFinally(bool $pick, bool $leave): void
{
    $value = 123;
    switch ($pick) {
        case true:
            if ($leave) {
                $value = 124;
                break;
            }
            try {
                $value = 125;
            } finally {
                $value = 126;
            }
            break;
        default:
            $value = 127;
    }
    $value; // observe:earlier-switch-break-tail
}
