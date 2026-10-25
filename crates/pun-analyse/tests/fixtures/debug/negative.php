<?php

namespace Shadow {
    function dumpType($value): void {}
    function dumpPhpDocType($value): void {}

    dumpType(1);
    dumpPhpDocType(2);
}

namespace DebugNegative {
    use function Shadow\dumpType;
    use function Shadow\dumpPhpDocType;

    dumpType(3);
    dumpPhpDocType(4);
    \NotLoaded\dumpType(5);
    $fn = 'PHPStan\\dumpType';
    $fn(6);
    $fn = 'PHPStan\\dumpPhpDocType';
    $fn(7);
    $callable = \PHPStan\dumpType(...);
    $callable(8);
    $callable = \PHPStan\dumpPhpDocType(...);
    $callable(9);

    class C
    {
        public function dumpType($value): void {}
        public function dumpPhpDocType($value): void {}
        public static function inspect(): void
        {
            $object = new self();
            $object->dumpType(10);
            $object->dumpPhpDocType(11);
        }
    }

    \PHPStan\Testing\assertType('1', 1);
    \PHPStan\Testing\assertType('int', 1);
    \PHPStan\dumpNativeType(12);
    \PHPStan\debugScope();
}
