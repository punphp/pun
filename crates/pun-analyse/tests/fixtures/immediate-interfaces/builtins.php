<?php
namespace ImmediateBuiltins;

class NativeArray extends \ArrayObject implements \ArrayAccess {
    public function offsetSet(string $key, mixed $value): void {}
}
class NativeGrandchild extends NativeArray implements \ArrayAccess {
    public function offsetSet(int $key, mixed $value): void {}
}
interface UserArray extends \ArrayAccess {
    public function offsetSet(mixed $key, mixed $value): void;
}
class TransitiveArray implements \ArrayAccess, UserArray {
    public function offsetExists(mixed $offset): bool { return false; }
    public function offsetGet(mixed $offset): mixed { return null; }
    public function offsetSet(string $key, mixed $value): void {}
    public function offsetUnset(mixed $offset): void {}
}
class NativeConstructor extends \ArrayObject implements \IteratorAggregate {
    public function __construct(int $only) {}
}
interface EnumContract { public function run(string $old): void; }
/** @no-named-arguments */
interface AEnumContract { public function run(string $old): void; }
enum Plain implements EnumContract, AEnumContract {
    case One;
    public function run(string $plain): void {}
}
enum Backed: string implements EnumContract, AEnumContract {
    case One = 'one';
    public function run(string $backed): void {}
}
function calls(EnumContract $enum): void {
    $enum->run(old: 'enum interface selection retains implicit native interfaces');
}
