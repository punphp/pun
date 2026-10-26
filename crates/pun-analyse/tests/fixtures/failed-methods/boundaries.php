<?php
namespace FailedMethodBoundaries;

class Known {
    public function value(): int { return 31; }
    public function child(): ?self { return null; }
    private function hidden(): string { return 'hidden'; }
    public static function staticValue(): string { return 'static'; }
    public function stop(): never { throw new \Exception(); }
}
class Other {}
/**
 * @method string documented()
 * @method static int documentedStatic()
 */
class Magic {
    public function __call(string $name, array $args): string { return 'magic'; }
    public static function __callStatic(string $name, array $args): int { return 7; }
}
function fallbackCopies(Known $known, object $object, mixed $mixed, int $int): void {
    $copy = $known->absent(\PHPStan\dumpType($int));
    \PHPStan\dumpType($copy);
    \PHPStan\dumpType($copy->again());
    $copy = 41;
    \PHPStan\dumpType($copy);
    \PHPStan\dumpType($object->absent());
    \PHPStan\dumpType($mixed->absent());
    \PHPStan\dumpType($known->hidden());
}
function nullableChains(?Known $known): void {
    \PHPStan\dumpType($known?->value());
    \PHPStan\dumpType($known?->child()?->value());
    \PHPStan\dumpType($known?->absent());
    \PHPStan\dumpType((null)?->absent());
}
function unions(Known|Other $known, Unknown|Other $missing, Known|int $scalar): void {
    \PHPStan\dumpType($known->value());
    \PHPStan\dumpType($missing->value());
    \PHPStan\dumpType($scalar->value());
}
function selectors(Known $known, object $object, mixed $mixed, string $method): void {
    \PHPStan\dumpType($known->$method());
    \PHPStan\dumpType($object->$method());
    \PHPStan\dumpType($mixed->$method());
    \PHPStan\dumpType(Known::$method());
}
function magic(Magic $magic): void {
    \PHPStan\dumpType($magic->documented());
    \PHPStan\dumpType(Magic::documentedStatic());
}
function staticReceivers(object $object, mixed $mixed, int $int, string $class): void {
    \PHPStan\dumpType($object::absent());
    \PHPStan\dumpType($mixed::absent());
    \PHPStan\dumpType($int::absent());
    \PHPStan\dumpType($class::absent());
}
function functions(mixed $mixed, object $object, string $name, callable $callable): void {
    $mixedResult = $mixed();
    $objectResult = $object();
    \PHPStan\dumpType($mixedResult);
    \PHPStan\dumpType($objectResult);
    \PHPStan\dumpType($name());
    \PHPStan\dumpType($callable());
    \PHPStan\dumpType((static fn (): int => 3)());
}
function neverResult(Known $known): void {
    \PHPStan\dumpType($known->value());
    $known->stop();
    \PHPStan\dumpType(123);
}
function neverReceiver(): void {
    $stop = static function (): never { throw new \Exception(); };
    \PHPStan\dumpType($stop()->absent());
    \PHPStan\dumpType(456);
}
function assertedMethods(object $object, Known $known): void {
    if (method_exists($object, 'absent')) {
        $result = $object->absent();
        \PHPStan\dumpType($result);
    }
    if (method_exists($known, 'absent')) {
        $result = $known->absent();
        \PHPStan\dumpType($result);
        \PHPStan\dumpType($known->value());
    }
}
