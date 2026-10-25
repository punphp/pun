<?php
namespace RenamedNamedArgument;

trait Caller {
    public function call(Contract $object): void {
        $object->run(value: 'inside a trait');
        // @phpstan-ignore argument.parameterRenamedInSubtype
        $object->run(value: 'trait comment versus using class file');
    }
}
trait Implementation {
    public function run(string $fromTrait, int $second = 0): void {}
}
trait AbstractPrototype {
    abstract public function method(string $original): void;
}
trait Body {
    public function method(string $original): void {}
}
