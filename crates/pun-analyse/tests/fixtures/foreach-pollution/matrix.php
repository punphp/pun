<?php
namespace ForeachPollution;

function guaranteed(bool $pick): void {
    $existing = 'before';
    if ($pick) { $possible = 'before'; }
    foreach (['value'] as $key => $value) {
        $existing = 'after';
        $possible = 'after';
        $created = 'body';
    }
    echo $key, $value, $created, $possible;
    \PHPStan\dumpType($existing);
    \PHPStan\dumpType($possible);
    \PHPStan\dumpType($created);
}

/** @param array<int, string> $items */
function possiblyEmpty(array $items): void {
    $existing = 'before';
    foreach ($items as $key => $value) {
        \PHPStan\dumpType($items);
        $existing = 'after';
        $created = 'body';
    }
    echo $key, $value, $created;
    \PHPStan\dumpType($existing);
    \PHPStan\dumpType($items);
}

function emptyLoop(bool $pick): void {
    $existing = 'before';
    if ($pick) { $possible = 'before'; }
    foreach ([] as $key => $value) { $existing = 'after'; $created = 'body'; }
    echo $key, $value, $created, $possible;
    \PHPStan\dumpType($existing);
}

function localBreak(bool $leave): void {
    $existing = 'before';
    foreach (['value'] as $key => $value) {
        if ($leave) { $existing = 'break'; $breakOnly = 1; break; }
        $existing = 'end';
        $endOnly = 1;
    }
    echo $key, $value, $breakOnly, $endOnly;
    \PHPStan\dumpType($existing);
}

function alwaysBreak(): void {
    $existing = 'before';
    foreach (['value'] as $key => $value) { $existing = 'break'; $created = 1; break; }
    echo $key, $value, $created;
    \PHPStan\dumpType($existing);
}

function localContinue(bool $skip): void {
    $existing = 'before';
    foreach (['value'] as $key => $value) {
        if ($skip) { $existing = 'continue'; $continueOnly = 1; continue; }
        $existing = 'end'; $endOnly = 1;
    }
    echo $key, $value, $continueOnly, $endOnly;
    \PHPStan\dumpType($existing);
}

function alwaysContinue(): void {
    $existing = 'before';
    foreach (['value'] as $key => $value) { $existing = 'continue'; $created = 1; continue; }
    echo $key, $value, $created;
    \PHPStan\dumpType($existing);
}

function conditionalReturn(bool $stop): void {
    $existing = 'before';
    foreach (['value'] as $key => $value) {
        if ($stop) { $existing = 'return'; return; }
        $existing = 'end'; $created = 1;
    }
    echo $key, $value, $created;
    \PHPStan\dumpType($existing);
}

/** @param array<int, string> $items */
function alwaysReturn(array $items): void {
    $existing = 'before';
    foreach ($items as $key => $value) { $existing = 'return'; $created = 1; return; }
    echo $key, $value, $created;
    \PHPStan\dumpType($existing);
    \PHPStan\dumpType($items);
}

function iterableAssignment(): void {
    foreach ($items = ['value'] as $key => $value) { $created = 1; }
    echo $items, $key, $value, $created;
    \PHPStan\dumpType($items);
}

function holderControl(bool $pick): void {
    if ($pick) { $guard = 'yes'; $held = 1; } else { $guard = 'no'; }
    foreach (['value'] as $value) { $created = 1; }
    if ($guard === 'yes') { echo $held; }
    echo $held, $created;
}

function unsetControl(): void {
    $existing = 'before';
    foreach (['value'] as $value) { unset($existing); }
    echo $existing, $value;
    \PHPStan\dumpType($existing);
}

function neighbor(): void {
    for ($initial = 0; false;) {}
    echo $initial;
    do { $do = 1; } while (false);
    { $block = 1; }
    echo $do, $block;
}

function nested(bool $leave): void {
    foreach (['outer'] as $outer) {
        foreach (['inner'] as $inner) {
            if ($leave) { $outward = 1; break 2; }
            $local = 1;
        }
        $afterInner = 1;
    }
    echo $outer, $inner, $outward, $local, $afterInner;
}

/** @param array<int, string> $items */
function replacedIterable(array $items): void {
    foreach ($items as $value) { $items = 'replaced'; }
    \PHPStan\dumpType($items);
}

/** @param non-empty-array<int, string> $items */
function nonemptyParameter(array $items): void {
    foreach ($items as $key => $value) { $created = 'body'; }
    echo $key, $value, $created;
    \PHPStan\dumpType($items);
}

/** @param array<int, string> $input */
function assignedIterable(array $input): void {
    foreach ($items = $input as $value) {
        \PHPStan\dumpType($items);
        $created = 1;
    }
    \PHPStan\dumpType($items);
    echo $created;
}

class Holder {
    /** @var array<int, string> */
    public array $items;
}

function propertyIterable(Holder $holder): void {
    foreach ($holder->items as $value) { \PHPStan\dumpType($holder->items); }
    \PHPStan\dumpType($holder->items);
}

function nestedContinue(bool $skip): void {
    foreach (['outer'] as $outer) {
        foreach (['inner'] as $inner) {
            if ($skip) { $outward = 1; continue 2; }
            $local = 1;
        }
        $afterInner = 1;
    }
    echo $outer, $inner, $outward, $local, $afterInner;
}

function guaranteedTermination(): void {
    foreach (['value'] as $value) { $created = 1; throw new \Exception(); }
    echo $value, $created;
}

function reassignedHolder(bool $pick): void {
    if ($pick) { $guard = 'yes'; $held = 1; } else { $guard = 'no'; }
    foreach (['value'] as $value) { $guard = 'yes'; }
    if ($guard === 'yes') { echo $held; }
}

function recreatedHolder(): void {
    $existing = 'before';
    unset($existing);
    foreach (['value'] as $value) { $existing = 'after'; }
    \PHPStan\dumpType($existing);
}

/** @param array<string, mixed> $input */
function extractBody(array $input): void {
    foreach (['value'] as $value) { extract($input); }
    echo $unknownAfterExtract;
}

/** @param \Traversable<int, string> $items */
function objectIterable(\Traversable $items): void {
    foreach ($items as $key => $value) { $created = 1; }
    echo $key, $value, $created;
    \PHPStan\dumpType($items);
}

function unrolledArrayHolders(): void {
    foreach ([['nested']] as $value) { $created = ['body']; }
    echo $value, $created;
    \PHPStan\dumpType($value);
    \PHPStan\dumpType($created);
}
