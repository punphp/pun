<?php
namespace RenamedNamedArgument;

class Base {
    public function method(string $original): void {}
    protected function protectedMethod(string $original): void {}
    private function privateMethod(string $original): void {}
    public static function staticMethod(string $original): void {}
    final public function finalMethod(string $original): void {}
    public function variadic(string ...$original): void {}
    public function inherited(string $original): void {}
    /** @no-named-arguments */
    public function forbidden(string $original): void {}
    public function inside(): void {
        $this->method(original: 'this');
        $this->protectedMethod(original: 'protected');
        $this->privateMethod(original: 'private');
        $this->staticMethod(original: 'instance static');
        $this->finalMethod(original: 'final method');
    }
}
class Middle extends Base {}
class Child extends Middle {
    public function method(string $renamed): void {}
    public function protectedMethod(string $renamed): void {}
    public function privateMethod(string $renamed): void {}
    public static function staticMethod(string $renamed): void {}
    public function variadic(string ...$renamed): void {}
    /** @no-named-arguments */
    public function inherited(string $renamed): void {}
    public function forbidden(string $renamed): void {}
}
class Grandchild extends Child {
    public function method(string $again): void {}
    public function inherited(string $again): void {}
}
/** @no-named-arguments */
class Forbidden {
    public function method(string $original): void {}
}
class AllowedChild extends Forbidden {
    public function method(string $renamed): void {}
}
/** @no-named-arguments */
class ForbiddenChild extends Base {
    public function method(string $renamed): void {}
}
final class FinalBase {
    public function method(string $original): void {}
}
/** @final */
class DocFinalBase {
    public function method(string $original): void {}
}
// Deliberately invalid overrides still exercise the collector gates, independently of the native errors.
class FinalChild extends FinalBase {
    public function method(string $renamed): void {}
}
class DocFinalChild extends DocFinalBase {
    public function method(string $renamed): void {}
}
class FinalMethodChild extends Base {
    public function finalMethod(string $renamed): void {}
}
function controls(Base $base, Child $child, Forbidden $forbidden, FinalBase $final, DocFinalBase $docFinal): void {
    $base->method(original: 'base');
    $base->variadic(original: 'named variadic', extra: 'extra binds original');
    $base->variadic(...['extra' => 'unpacked extra']);
    $base->method(original: 'duplicate', original: 'duplicate again');
    $base->method('positional', original: 'duplicate after positional');
    $base->method(unknown: 'unknown');
    $base->method('too', 'many', original: 'not checked after extra positional');
    $base->inherited(original: 'overriding method opts out, prototype does not');
    $child->inherited(renamed: 'inherited no-named');
    $base->forbidden(original: 'method opts out');
    $forbidden->method(original: 'class opts out');
    $final->method(original: 'final class');
    $docFinal->method(original: 'doc final class');
    Base::staticMethod(original: 'static call');
    $base->staticMethod(original: 'instance static call');
    $base->finalMethod(original: 'final method not final class');
    $exact = new Base();
    $exact->method(original: 'new is asFinal');
    (new Base())->method(original: 'inline new is asFinal');
    $callable = $base->method(...);
    $callable(original: 'callable does not collect');
    $child->method(renamed: 'direct child prototype');
}

/** @phpstan-consistent-constructor */
abstract class Consistent {
    abstract public function __construct(string $original);
}
class ConsistentChild extends Consistent {
    public function __construct(string $renamed) {}
}
function constructors(Consistent $object): void {
    $object->__construct(original: 'instance constructor');
    new ConsistentChild(renamed: 'new does not collect');
}

function exactInherited(): void {
    $middle = new Middle();
    $middle->method(original: 'new inherited');
    (new Middle())->method(original: 'inline new inherited');
}
