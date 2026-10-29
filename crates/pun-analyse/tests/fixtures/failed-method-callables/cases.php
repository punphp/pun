<?php
declare(strict_types=1);

class CallableTarget {
    public function known(string $value): int { return 1; }
}

function stringReceiver(): mixed {
    $receiver = 'hello';
    $closure = $receiver->missing(...);
    $post = $closure;
    $unknownResult = $closure(1, 'two');
    $continued = 41;
    return [$post, $unknownResult, $continued];
}
function integerReceiver(): mixed {
    $receiver = 42;
    $closure = $receiver->missing(...);
    $post = $closure;
    $continued = 42;
    return [$post, $continued];
}
function nullReceiver(): mixed {
    $receiver = null;
    $closure = $receiver->missing(...);
    $post = $closure;
    $continued = 43;
    return [$post, $continued];
}
function missingMethod(CallableTarget $receiver): mixed {
    $closure = $receiver->missing(...);
    $post = $closure;
    $continued = 44;
    return [$post, $continued];
}
function dynamicInteger(CallableTarget $receiver): mixed {
    $name = 123;
    $closure = $receiver->$name(...);
    $post = $closure;
    $continued = 45;
    return [$post, $continued];
}
function dynamicEffects(): mixed {
    $closure = ($receiver = new CallableTarget())->{($name = 123)}(...);
    $post = $closure;
    $receiverAfter = $receiver;
    $selectorAfter = $name;
    $continued = 46;
    return [$post, $receiverAfter, $selectorAfter, $continued];
}
function dynamicKnown(CallableTarget $receiver): mixed {
    $name = 'known';
    $closure = $receiver->{$name}(...);
    $post = $closure;
    $unknownResult = $closure(1, 'two');
    $continued = 47;
    return [$post, $unknownResult, $continued];
}
function dynamicNonObject(): mixed {
    $closure = 'hello'->{123}(...);
    $post = $closure;
    $continued = 48;
    return [$post, $continued];
}
function knownMethod(CallableTarget $receiver): mixed {
    $closure = $receiver->known(...);
    $post = $closure;
    $result = $closure('value');
    $continued = 49;
    return [$post, $result, $continued];
}
