<?php

namespace ClosureFromCallableControls;

class ParentTransformer {
    public static function inherited(string $value): string { return $value; }
    protected static function protectedMethod(string $value): string { return $value; }
    private static function privateMethod(string $value): string { return $value; }
    public function instance(string $value): string { return $value; }
    public static function inside(): void {
        \PHPStan\dumpType(\Closure::fromCallable([self::class, 'privateMethod']));
        \PHPStan\dumpType(\Closure::fromCallable([self::class, 'protectedMethod']));
    }
}

class ChildTransformer extends ParentTransformer {
    public static function insideChild(): void {
        \PHPStan\dumpType(\Closure::fromCallable([self::class, 'protectedMethod']));
    }
}

function inherited(ChildTransformer $object): void {
    \PHPStan\dumpType(\Closure::fromCallable([ChildTransformer::class, 'inherited']));
    \PHPStan\dumpType(\Closure::fromCallable('ClosureFromCallableControls\\ChildTransformer::inherited'));
    \PHPStan\dumpType(\Closure::fromCallable([$object, 'inherited']));
    \PHPStan\dumpType(\Closure::fromCallable([$object, 'instance']));
}

function invalid(): void {
    \Closure::fromCallable();
    \Closure::fromCallable('strlen', 'extra');
    \Closure::fromCallable(callback: 123);
    \Closure::fromCallable('ClosureFromCallableControls\\missing');
    \Closure::fromCallable([ParentTransformer::class, 'missing']);
    \Closure::fromCallable([ParentTransformer::class, 'privateMethod']);
    \Closure::fromCallable([ParentTransformer::class, 'protectedMethod']);
    \Closure::fromCallable([ParentTransformer::class, 'instance']);
}

function bindingControls(): void {
    $static = static fn(string $value): string => $value;
    \PHPStan\dumpType(\Closure::bind($static, new \stdClass()));
    \PHPStan\dumpType($static->bindTo(new \stdClass()));
    $nonStatic = fn(string $value): string => $value;
    \PHPStan\dumpType(\Closure::bind($nonStatic, null));
    \PHPStan\dumpType($nonStatic->bindTo(null));
}
