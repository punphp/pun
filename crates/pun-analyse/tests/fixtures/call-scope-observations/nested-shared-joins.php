<?php

final class NestedJoinSink {}
final class NestedJoinInspect {
    public function inspect($root, $parent, $value) {}
}

/**
 * @param array{nest: array{sink: NestedJoinSink|null}, other: 0} $broad
 * @param array{nest: array{sink: NestedJoinSink}, other: 1} $narrow
 */
function overlappingNestedScopes(bool $branch, array $broad, array $narrow): void {
    $inspect = new NestedJoinInspect();
    if ($branch) {
        $grid = $broad;
    } else {
        $grid = $narrow;
    }
    $inspect->inspect($grid, $grid['nest'], $grid['nest']['sink']); // observe
    if ($branch) {
        $reverse = $narrow;
    } else {
        $reverse = $broad;
    }
    $inspect->inspect($reverse, $reverse['nest'], $reverse['nest']['sink']); // observe
}

/**
 * @param array{nest: array{sink: null}, other: 0} $empty
 * @param array{nest: array{sink: NestedJoinSink}, other: 1} $full
 */
function unrelatedNestedScopes(bool $branch, array $empty, array $full): void {
    $inspect = new NestedJoinInspect();
    if ($branch) {
        $grid = $empty;
    } else {
        $grid = $full;
    }
    $inspect->inspect($grid, $grid['nest'], $grid['nest']['sink']); // observe
    if ($branch) {
        $reverse = $full;
    } else {
        $reverse = $empty;
    }
    $inspect->inspect($reverse, $reverse['nest'], $reverse['nest']['sink']); // observe
}
