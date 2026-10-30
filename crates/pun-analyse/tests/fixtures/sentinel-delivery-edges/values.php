<?php
namespace {
    const GLOBAL_SHAPE = ['z' => 1, 'a' => 2];
    define('DEFINED_SHAPE', ['m' => 3, 'b' => 4]);
}
namespace SentinelDeliveryEdges {
    use function PHPStan\dumpType;

    const USER_KEY = 'value';
    class Values {
        const GLOBAL_COPY = GLOBAL_SHAPE;
        const DEFINED_COPY = DEFINED_SHAPE;
        const UNION_EXPRESSION = ['a' => 2, 'z' => 1] + [];
        const DUPLICATE = ['z' => ['old' => 0], 'a' => 2, 'z' => ['new' => 3]];
        const KEY = 'value';
        const USER_SHAPE = [USER_KEY => 1];
    }
    dumpType(Values::GLOBAL_COPY, Values::DEFINED_COPY, Values::DUPLICATE);
    dumpType(Values::UNION_EXPRESSION, Values::USER_SHAPE);

    /** @param array{only: int, ...<string, int>} $shape */
    function openStrings(array $shape): array {
        unset($shape['only']);
        dumpType($shape);
        return $shape;
    }
    /** @param array{only: int, ...<int, int>} $shape */
    function openIntegers(array $shape): array {
        unset($shape['only']);
        dumpType($shape);
        return $shape;
    }

    class Receiver {
        /** @param array{value: int} $input */
        function consume(array $input): void {}
        function named(int $value): void {}
    }
    class Child extends Receiver {
        function named(int $renamed): void {}
    }
    function selected(Receiver $receiver, string $key, object $unknown): void {
        $receiver->consume([Values::KEY => 1]);
        $receiver->consume([$key => 1]);
        $copy = ['value' => 1];
        $receiver->consume($copy);
        $receiver->named(...[Values::KEY => 1]);
        $key = 'value';
        $receiver->named(...[$key => 1]);
        $receiver->named(...$copy);
        $class = Values::class;
        $receiver->named(...[$class::KEY => 1]);
        $key = PHP_DEBUG;
        $receiver->named(...[$key => 1]);
        $class = get_class($unknown);
        $receiver->named(...[$class::KEY => 1]);
        $receiver->named(...Values::USER_SHAPE);
    }

    /** @return int */
    function unavailableMatch() {
        return match (true) { (yield) => 1, default => 0 };
    }
}
