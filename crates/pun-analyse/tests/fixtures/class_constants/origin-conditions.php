<?php

namespace ConstantOriginConditions;

function incoming(mixed $receiver, bool $flag): void {
    if ($flag) { echo $receiver::YES; } else { echo $receiver::NO; }
    echo $receiver::AFTER;
}

function mutatedBranch(mixed $receiver, $implicit, bool $flag): void {
    if ($flag) { $receiver = $implicit; echo $receiver::YES; }
    else { echo $receiver::NO; }
    echo $receiver::AFTER;
}

function mutatedCondition(mixed $receiver, $implicit): void {
    if (($receiver = $implicit) !== null) { echo $receiver::YES; }
    else { echo $receiver::NO; }
    echo $receiver::AFTER;
}

function returningMutation(mixed $receiver, $implicit, bool $flag): void {
    if ($flag) { $receiver = $implicit; return; }
    echo $receiver::AFTER;
}

function elseifIncoming(mixed $receiver, $implicit, bool $flag, bool $other): void {
    if ($flag) { $receiver = $implicit; }
    elseif ($other) { echo $receiver::ELSEIF; }
    else { echo $receiver::ELSE; }
    echo $receiver::AFTER;
}

function colonBranches(mixed $receiver, bool $flag, bool $other): void {
    if ($flag):
        echo $receiver::YES;
    elseif ($other):
        echo $receiver::ELSEIF;
    else:
        echo $receiver::NO;
    endif;
    echo $receiver::AFTER;
    if ($flag): echo $receiver::NO_ELSE; endif;
}

function conditionalExpressions(mixed $receiver, bool $flag, $fallback): void {
    $flag ? print($receiver::YES) : print($receiver::NO);
    $flag ?: print($receiver::SHORT);
    $flag && print($receiver::AND);
    $flag || print($receiver::OR);
    $fallback ?? print($receiver::COALESCE);
    echo $receiver::AFTER;
}

function conditionalMutation(mixed $receiver, $implicit, bool $flag): void {
    $flag ? ($receiver = $implicit) : print($receiver::NO);
    echo $receiver::AFTER;
}

/** @param mixed $receiver */
function annotated($receiver, bool $flag): void {
    if ($flag) { echo $receiver::YES; } else { echo $receiver::NO; }
}

function unsetOrigin(mixed $receiver, mixed $untouched, $implicit, bool $flag): void {
    if ($flag) { unset($receiver); $receiver = $implicit; echo $receiver::YES; }
    else { echo $receiver::NO; }
    echo $receiver::AFTER;
    echo $untouched::AFTER;
}

function allTerminate(mixed $receiver, bool $flag): void {
    if ($flag) { echo $receiver::YES; return; } else { echo $receiver::NO; throw new \Exception; }
}

function reversedMutation(mixed $receiver, $implicit, bool $flag): void {
    if ($flag) { echo $receiver::YES; } else { $receiver = $implicit; echo $receiver::NO; }
    echo $receiver::AFTER;
}

function bothMutate(mixed $receiver, $implicit, bool $flag): void {
    if ($flag) { $receiver = $implicit; } else { $receiver = $implicit; }
    echo $receiver::AFTER;
}

function copiedOrigin(mixed $receiver, bool $flag): void {
    if ($flag) { $receiver = $receiver; }
    echo $receiver::AFTER;
}

function nativeReceiver(int $receiver, $implicit, bool $flag): void {
    if ($flag) { $receiver = $implicit; } else { echo $receiver::NO; }
    echo $receiver::AFTER;
}

function shortCircuitMutation(mixed $receiver, $implicit, bool $flag): void {
    $flag && ($receiver = $implicit);
    echo $receiver::AND;
}
