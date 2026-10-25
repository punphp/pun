<?php

namespace VariableFlow;

function breaks(int $max, bool $flag): void
{
    $i = 0;
    while (true) {
        if ($i > $max) {
            $while = 'done';
            break;
        }
        ++$i;
    }
    echo $while;
    compact('while');
    for ($i = 0;; ++$i) {
        if ($i > $max) {
            $for = 'done';
            break;
        }
    }
    echo $for;
    do {
        if ($i > $max) {
            $do = 'done';
            break;
        }
        ++$i;
    } while (true);
    echo $do;
    while (true) {
        if ($flag) {
            $sometimes = 1;
            break;
        }
        break;
    }
    echo $sometimes;
    compact('sometimes');
    while (true) {
        if ($flag) {
            break;
        }
        $otherEdge = 1;
        break;
    }
    echo $otherEdge;
    compact('otherEdge');
    while ($flag) {
        $skipped = 1;
        break;
    }
    echo $skipped;
    compact('skipped');
}

function continued(bool $flag): void
{
    for ($i = 0; $i < 3; $i++) {
        try {
            if ($flag) {
                return;
            }
            throw new \Exception();
        } catch (\Exception $caught) {
            continue;
        }
    }
    echo $caught;
    compact('caught');
    for ($i = 0; $i < 3; $i++) {
        if ($flag) {
            continue;
        }
        $fallthrough = 1;
    }
    echo $fallthrough;
    compact('fallthrough');
    for ($i = 0; $i < 3; $i++) {
        if ($flag) {
            $continued = 1;
            continue;
        }
    }
    echo $continued;
    compact('continued');
}

function switchEffects(string $input): void
{
    switch (true) {
        case strpos($input, 'get_') === 0 && preg_match('/get_(\S+)/', $input, $matches) === 1:
            echo $matches;
            compact('matches');
            break;
        default:
            echo $notAssigned;
            echo $matches;
            compact('matches');
    }
    switch (true) {
        case ($assigned = $input) === 'a':
            echo $assigned;
            break;
        default:
            echo $assigned;
    }
    echo $assigned;
}

function nullable(mixed $value, bool $flag): void
{
    if (isset($value)) {
        echo $value;
    }
    if (empty($value)) {
        echo $value;
    }
    echo $value;
    compact('value');
    if ($flag) {
        $maybe = 1;
    }
    if (isset($maybe)) {
        echo $maybe;
    }
    echo $maybe;
    compact('maybe');
    if (isset($missing)) {
        echo $missing;
    }
    echo $missing;
    compact('missing');
}

function absentDeclaration(): void
{
    /** @var string $name */
    echo $name;
    compact('name');
}

function includedDeclaration(): void
{
    require 'who-to-love.php';
    /** @var string $name */
    echo $name;
    compact('name');
    echo $unknown;
    compact('unknown');
}

function maybeDeclaration(bool $flag): void
{
    if ($flag) {
        $name = 1;
    }
    /** @var string $name */
    echo $name;
    compact('name');
}

function definedDeclaration(): void
{
    $name = 1;
    /** @var string $name */
    echo $name;
    compact('name');
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
    if ($name === 'left') {
        echo $left, $right;
        compact('left', 'right');
    } else {
        echo $left, $right;
        compact('left', 'right');
    }
}
