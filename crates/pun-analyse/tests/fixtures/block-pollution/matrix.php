<?php
namespace BlockPollution;

function asymmetric(bool $pick): void {
    $existing = 1;
    if ($pick) { $possible = 'entry'; }
    {
        \PHPStan\dumpType($existing);
        $existing = 'body';
        $possible = 2;
        $created = ['body'];
        \PHPStan\dumpType($existing);
        echo $created, $possible;
    }
    echo $created, $possible;
    \PHPStan\dumpType($existing);
    \PHPStan\dumpType($possible);
    \PHPStan\dumpType($created);
    $surrounding = 'outside';
    \PHPStan\dumpType($surrounding);
    echo $neverAssigned;
}

function nested(): void {
    $existing = 'entry';
    {
        $existing = 1;
        { $existing = false; $nested = 'nested'; }
        \PHPStan\dumpType($existing);
        echo $nested;
        $outer = 2;
    }
    echo $nested, $outer;
    \PHPStan\dumpType($existing);
}

function conditional(bool $pick, bool $stop): void {
    $existing = 'entry';
    {
        if ($stop) { $existing = 'return'; return; }
        if ($pick) { $created = 1; }
        $existing = 'end';
    }
    echo $created;
    \PHPStan\dumpType($existing);
}

function returns(): void {
    { $created = 1; return; }
    echo $unreachableAfterReturn;
    \PHPStan\dumpType($created);
}

function throws(): void {
    $existing = 'entry';
    try {
        { $existing = 'throw'; $thrown = 1; throw new \RuntimeException(); }
        echo $unreachableAfterThrow;
    } catch (\RuntimeException $caught) {
        echo $thrown;
        \PHPStan\dumpType($existing);
    }
    echo $thrown;
    \PHPStan\dumpType($existing);
}

function localBreak(bool $leave): void {
    $existing = 'entry';
    foreach (['item'] as $value) {
        {
            if ($leave) { $existing = 'break'; $onBreak = 1; break; }
            $existing = 'end'; $onEnd = 2;
        }
        \PHPStan\dumpType($existing);
    }
    echo $onBreak, $onEnd, $value;
    \PHPStan\dumpType($existing);
}

function alwaysBreak(): void {
    $existing = 'entry';
    foreach (['item'] as $value) {
        { $existing = 'break'; $created = 1; break; }
        echo $unreachableAfterBreak;
    }
    echo $created;
    \PHPStan\dumpType($existing);
}

function alwaysContinue(): void {
    $existing = 'entry';
    foreach (['item'] as $value) {
        { $existing = 'continue'; $created = 1; continue; }
        echo $unreachableAfterContinue;
    }
    echo $created;
    \PHPStan\dumpType($existing);
}

function outward(bool $leave): void {
    foreach (['outer'] as $outer) {
        foreach (['inner'] as $inner) {
            {
                if ($leave) { $breakOut = 1; break 2; }
                $continueOut = 2;
                continue 2;
            }
            echo $unreachableInner;
        }
        echo $unreachableOuter;
    }
    echo $outer, $inner, $breakOut, $continueOut;
}

function holderPreserved(bool $pick): void {
    if ($pick) { $guard = 'yes'; $held = 1; } else { $guard = 'no'; }
    { $unrelated = 2; }
    if ($guard === 'yes') { echo $held; }
    echo $held, $unrelated;
}

function holderInvalidated(bool $pick): void {
    if ($pick) { $guard = 'yes'; $held = 1; } else { $guard = 'no'; }
    { $guard = 'changed'; }
    if ($guard === 'yes') { echo $held; }
    echo $held;
    \PHPStan\dumpType($guard);
}

function unsetAndExtract(array $input): void {
    $existing = 'entry';
    { unset($existing); $created = 1; }
    echo $existing, $created;
    \PHPStan\dumpType($existing);
    { extract($input); }
    echo $fromExtract;
}

function expressions(string $input): void {
    $count = 1;
    {
        $count++;
        preg_match('/x/', $input, $matches);
    }
    \PHPStan\dumpType($count);
    echo $matches;
}

function independent(): void {
    for ($initial = 0; false;) {}
    echo $initial;
    while (($whileInitial = false)) {}
    echo $whileInitial;
    foreach (['item'] as $key => $value) { $foreachCreated = 1; }
    echo $key, $value, $foreachCreated;
    do { $do = 1; } while (false);
    echo $do;
    if (true) { $if = 1; }
    echo $if;
    { $block = 1; }
    echo $block;
}

/** @param array<int, string> $items */
function nestedLoops(array $items): void {
    {
        for ($initial = 0; false;) {}
        foreach ($items as $value) { \PHPStan\dumpType($items); $created = 1; }
    }
    echo $initial, $value, $created;
    \PHPStan\dumpType($items);
}

function includeAndMatch(): void {
    $file = 'pun-block-missing-entry.php';
    $subject = 1;
    { $file = 'pun-block-missing-body.php'; $subject = 'body'; }
    include $file;
    $result = match ($subject) { 1 => 'one', 'body' => 'body' };
    \PHPStan\dumpType($result);
}

function bodyRoles(bool $pick, bool $other): void {
    if ($pick) { $merged = 'if'; } elseif ($other) { $merged = 'elseif'; } else { $merged = 'else'; }
    \PHPStan\dumpType($merged);
    if ($pick) $naked = 'if'; else $naked = 'else';
    echo $naked;
    if (true): { $colonIf = 1; } endif;
    echo $colonIf;
    for ($i = 0; $i < 1; $i++): { $colonFor = 1; } endfor;
    echo $colonFor;
    while (true): { $colonWhile = 1; } break; endwhile;
    echo $colonWhile;
    foreach (['item'] as $value): { $colonForeach = 1; } endforeach;
    echo $colonForeach;
    do { $doBody = 1; { $doBlock = 2; } } while (false);
    echo $doBody, $doBlock;
    do $doNaked = 1; while (false);
    echo $doNaked;
    while (true) { $whileBody = 1; break; }
    echo $whileBody;
}
