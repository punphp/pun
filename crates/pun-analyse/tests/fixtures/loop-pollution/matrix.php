<?php

namespace LoopPollution;

function zeroFor(bool $pick): void {
    $existing = 'before';
    if ($pick) { $maybe = 1; }
    for ($initial = 1, $existing = 'init', $maybe = 'init'; $condition = false; $update = 1) {
        $body = 1;
    }
    \PHPStan\dumpType($existing);
    \PHPStan\dumpType($maybe);
    \PHPStan\dumpType($initial);
    \PHPStan\dumpType($condition);
    echo $body, $update;
}

function maybeFor(bool $enter, bool $pick): void {
    $existing = 'before';
    if ($pick) { $maybe = 1; }
    for ($initial = 1, $maybe = 'init'; $condition = $enter; $update = 1) {
        $existing = 'body';
        $body = 1;
        break;
    }
    \PHPStan\dumpType($existing);
    \PHPStan\dumpType($maybe);
    \PHPStan\dumpType($initial);
    \PHPStan\dumpType($condition);
    echo $body, $update;
}

function guaranteedFor(bool $pick): void {
    $existing = 5;
    if ($pick) { $maybe = 1; }
    for ($initial = 1, $maybe = 'init'; $condition = true;) {
        $existing = 'body';
        $body = 1;
        break;
    }
    \PHPStan\dumpType($existing);
    \PHPStan\dumpType($maybe);
    \PHPStan\dumpType($initial);
    \PHPStan\dumpType($condition);
    echo $body;
}

function zeroWhile(bool $pick): void {
    $existing = 'before';
    if ($pick) { $maybe = 1; }
    while (($condition = false) && ($existing = 'condition') && ($maybe = 'condition')) {
        $body = 1;
    }
    \PHPStan\dumpType($existing);
    \PHPStan\dumpType($maybe);
    \PHPStan\dumpType($condition);
    echo $body;
}

function maybeWhile(bool $enter): void {
    $existing = 'before';
    while ($condition = $enter) {
        $body = 1;
        $existing = 'body';
        break;
    }
    \PHPStan\dumpType($existing);
    \PHPStan\dumpType($condition);
    echo $body;
}

function guaranteedWhile(bool $pick): void {
    $existing = 5;
    if ($pick) { $maybe = 1; }
    while ($condition = true) {
        $existing = 'body';
        $maybe = 'body';
        $body = 1;
        break;
    }
    \PHPStan\dumpType($existing);
    \PHPStan\dumpType($maybe);
    \PHPStan\dumpType($condition);
    echo $body;
}

function updates(): void {
    for ($i = 0; $i < 2; $i++, $update = 'update') {
        $body = 1;
        continue;
    }
    \PHPStan\dumpType($update);
    echo $i, $body;
}

function nestedBreak(): void {
    for ($outer = 0;; $outer++) {
        while ($inner = true) {
            $escaped = 'break';
            break 2;
        }
    }
    \PHPStan\dumpType($outer);
    \PHPStan\dumpType($inner);
    \PHPStan\dumpType($escaped);
}

function nestedContinue(): void {
    for ($outer = 0; $outer < 2; $outer++, $update = 'outer') {
        for ($inner = 0;;) {
            $continued = 'continue';
            continue 2;
        }
    }
    \PHPStan\dumpType($update);
    echo $outer, $continued;
}

function terminating(bool $enter): void {
    for ($initial = 1; $condition = $enter; $update = 1) {
        return;
    }
    echo $initial, $condition, $update;
    while ($whileCondition = $enter) {
        throw new \Exception();
    }
    echo $whileCondition;
}

function controls(): void {
    do { $fromDo = 1; } while ($doCondition = false);
    foreach ([1] as $key => $value) { $fromForeach = 1; }
    { $fromBlock = 1; }
    echo $fromDo, $doCondition, $key, $value, $fromForeach, $fromBlock;
}

function commaCondition(): void {
    for ($initial = 'init'; $discarded = false, $condition = true; $update = 1) {
        $body = 'body';
        break;
    }
    \PHPStan\dumpType($discarded);
    \PHPStan\dumpType($condition);
    echo $initial, $body, $update;
}

function conditionWrites(bool $enter): void {
    $existing = 'before';
    while (($condition = $enter) && ($existing = 'condition')) {
        $body = 1;
        break;
    }
    \PHPStan\dumpType($existing);
    \PHPStan\dumpType($condition);
    echo $body;
}

function falseWhile(): void {
    while ($condition = false) { $body = 1; }
    \PHPStan\dumpType($condition);
    echo $body;
}

function terminatingWrites(bool $enter): void {
    $existing = 'before';
    for ($initial = 1; $condition = $enter; $update = 1) {
        $existing = 'body';
        throw new \Exception();
    }
    \PHPStan\dumpType($existing);
    echo $initial, $condition, $update;
    while ($whileCondition = $enter) {
        $existing = 'while';
        return;
    }
    \PHPStan\dumpType($existing);
    echo $whileCondition;
}

function conditionalHolder(bool $enter): void {
    $guard = 0;
    for (; $enter;) {
        $guard = 1;
        $value = 'loop';
        break;
    }
    if ($guard === 1) {
        \PHPStan\dumpType($value);
        echo $value;
    }
    echo $value;
}

function conditionalBreakFinite(bool $enter, bool $leave): void {
    $existing = 1;
    for ($i = 0; $condition = $enter; $i++, $update = 'update') {
        if ($leave) {
            $existing = 'break';
            $branchOnly = 1;
            break;
        }
        $existing = 'back';
        $enter = false;
    }
    \PHPStan\dumpType($existing);
    \PHPStan\dumpType($condition);
    \PHPStan\dumpType($update);
    echo $i, $branchOnly;
}

function guaranteedReturn(): void {
    for ($initial = 1; true; $update = 1) { return; }
    echo $initial, $update, $unreachable;
}

function guaranteedThrow(): void {
    while ($condition = true) { throw new \Exception(); }
    echo $condition, $unreachable;
}

function conditionalBreakInvariant(bool $enter, bool $leave): void {
    $existing = 1;
    for ($i = 0; $condition = $enter; $i++, $update = 'update') {
        if ($leave) {
            $existing = 'break';
            $branchOnly = 1;
            break;
        }
        $existing = 'back';
    }
    \PHPStan\dumpType($existing);
    \PHPStan\dumpType($condition);
    \PHPStan\dumpType($update);
    echo $i, $branchOnly;
}

function updateExisting(bool $enter, bool $leave): void {
    $update = 'before';
    for (; $enter; $update = 'updated') {
        if ($leave) { break; }
        $enter = false;
    }
    \PHPStan\dumpType($update);
}

function updateExistingUnion(bool $pick, bool $enter, bool $leave): void {
    $update = $pick ? 'before' : 1;
    for (; $enter; $update = 'updated') {
        if ($leave) { break; }
        $enter = false;
    }
    \PHPStan\dumpType($update);
}

function deadUpdateRead(bool $enter): void {
    for ($initial = 1; $condition = $enter; $initial = $unknownUpdate) { break; }
    \PHPStan\dumpType($initial);
}

function terminatingUpdateRead(bool $enter): void {
    for ($initial = 1; $condition = $enter; $initial = $unknownAfterReturn) { return; }
    \PHPStan\dumpType($initial);
}

function replacedParent(string $key, bool $enter, bool $leave): void {
    $values = [];
    for (; $enter; $values = []) {
        $values[$key] = 1;
        if ($leave) { break; }
        $enter = false;
    }
    \PHPStan\dumpType($values[$key] ?? 0);
}
