<?php
namespace UnreachableWalk;

function terminated(): int {
    return 1;
    missing();
    'unreachable';
}

function labeled(): int {
    goto alive;
    missing();
    alive:
    'reachable';
    return 2;
}

function declarations(): int {
    return 3;
    function nested(): int {
        'nested-reachable';
        return 4;
        missing();
    }
    'unreachable';
}

exit;
function earlyBound(): int {
    'early-bound-reachable';
    return 5;
}
class EarlyBoundClass {
    public function check(): int {
        'class-reachable';
        return 6;
    }
}
missing();
