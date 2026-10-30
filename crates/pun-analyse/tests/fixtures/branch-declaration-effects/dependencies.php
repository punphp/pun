<?php declare(strict_types=1);

namespace BranchDeclarationDependencies;

/** @param array{item: bool} $values */
function parameterArray(bool $flag, array $values): void {
    if ($flag) {
        \PHPStan\dumpType($values['item']);
    }
}
function nonemptyCondition(): void { if ($a = [1]) { $x = 1; } else { echo 'unreachable'; } $a[] = 2; }
function legacyCondition(): void { if ($a = array()) { echo 'unreachable'; } $a[] = 1; }
function negatedLiteral(): void { if (!(false)) { $x = 1; } else { echo 'unreachable'; } }
function falseEffect(): void { if ((print 'effect') === 0) { $x = 1; } }
function throwingBranch(bool $flag, \Throwable $exception): void { if ($flag) { throw $exception; } else { $x = 1; } }
function caughtBranch(bool $flag, \Throwable $exception): void {
    try {
        if ($flag) { throw $exception; } else { $x = 1; }
    } catch (\Throwable $caught) {
    }
}

function unsupportedElse(bool $flag): void { if ($flag) { $x = 1; } else { strlen('text'); } }
/** @param \ArrayAccess<int, int> $external */
function objectOffsetRead(bool $flag, \ArrayAccess $external): void { if ($flag) { $x = $external[0]; } }
