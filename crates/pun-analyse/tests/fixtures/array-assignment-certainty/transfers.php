<?php

namespace ArrayAssignmentCertainty;

/**
 * @param array<string, list<int>> $groups
 * @param list<int> $replacement
 */
function mutate(array $groups, string $key, array $replacement): void
{
    if (!isset($groups[$key])) {
        $groups[$key] = $replacement;
        array_unshift($groups[$key], 1);
        \PHPStan\dumpType($groups[$key]);
    }
}

/**
 * @param array<string, array<string, list<int>>> $groups
 * @param list<int> $replacement
 */
function nested(array $groups, string $outer, string $inner, array $replacement): void
{
    if (!isset($groups[$outer][$inner])) {
        $groups[$outer][$inner] = $replacement;
        array_unshift($groups[$outer][$inner], 1);
        \PHPStan\dumpType($groups[$outer][$inner]);
    }
}

function unrelated(bool $flag): void
{
    if ($flag) {
        $tag = 'left';
        $left = 1;
    } else {
        $tag = 'right';
        $right = 1;
    }
    $written = [];
    $written[] = 1;
    if ($tag === 'left') {
        echo $left, $right;
        compact('left', 'right');
    }
}

function mutatedGuard(bool $flag): void
{
    if ($flag) {
        $guard = ['tag' => 'left'];
        $left = 1;
    } else {
        $guard = ['tag' => 'right'];
        $right = 1;
    }
    $guard['tag'] = 'left';
    if ($guard['tag'] === 'left') {
        echo $left;
        compact('left');
    }
}

function undefinedIndex(): void
{
    $written = [];
    $written[$missing][] = 1;
    compact('written');
}
