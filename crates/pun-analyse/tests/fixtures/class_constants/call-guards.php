<?php

namespace ConstantCallGuards;

use function ConstantCallGuards\pureReceiver as imported;

class Receiver {
    private const PRIVATE_NAME = 1;
}

/** @phpstan-pure */
function pureReceiver(int $index = 0): Receiver { return new Receiver; }
/** @phpstan-impure */
function impureReceiver(int $index = 0): Receiver { return new Receiver; }
function maybeReceiver(int $index = 0): Receiver { return new Receiver; }
/** @phpstan-impure */
function noise(): int { return 1; }
/** @phpstan-pure */
function value(mixed $value): Receiver { return new Receiver; }
function replace(int &$index): void { $index = 1; }

class Factory {
    public function __construct() {}
    /** @phpstan-pure */
    public function pure(int $index = 0): Receiver { return new Receiver; }
    /** @phpstan-impure */
    public function impure(): Receiver { return new Receiver; }
    public function maybe(): Receiver { return new Receiver; }
    /** @phpstan-pure */
    public static function pureStatic(int $index = 0): Receiver { return new Receiver; }
    /** @phpstan-impure */
    public static function impureStatic(): Receiver { return new Receiver; }
    public static function maybeStatic(): Receiver { return new Receiver; }
    public function mutate(): void {}
    /** @phpstan-pure */
    public function pureVoid(): void {}
    public function fluent(): self { return $this; }
    /** @phpstan-pure */
    public function pureFluent(): self { return $this; }

    /** @return $this */
    public function thisFluent(): self { return $this; }
    public function replace(int &$index): void { $index = 1; }
    public static function replaceStatic(int &$index): void { $index = 1; }

    public function lexical(): void {
        if (self::pureStatic()::LEXICAL) {
            echo self::pureStatic()::LEXICAL;
            self::impureStatic();
            echo self::pureStatic()::LEXICAL;
        }
        if ($this->pure()::EXPLICIT) {
            Factory::impureStatic();
            echo $this->pure()::EXPLICIT;
        }
        if (FACTORY::pureStatic()::CASE_NAME) {
            Factory::impureStatic();
            echo FACTORY::pureStatic()::CASE_NAME;
        }
    }
}

function functions(): void {
    if (pureReceiver()::MISSING) {
        echo pureReceiver()::MISSING;
        echo imported()::MISSING;
        noise();
        echo pureReceiver()::MISSING;
    }
    if (impureReceiver()::MISSING) { echo impureReceiver()::MISSING; }
    if (maybeReceiver()::MISSING) { echo maybeReceiver()::MISSING; }
    if (pureReceiver()::PRIVATE_NAME) { echo pureReceiver()::PRIVATE_NAME; }
    if (pureReceiver()::OTHER) { echo impureReceiver()::OTHER; }
}

function methods(Factory $factory, Factory $other): void {
    if ($factory->pure()::MISSING) {
        echo $factory->pure()::MISSING;
        $other->mutate();
        echo $factory->pure()::MISSING;
        $factory->mutate();
        echo $factory->pure()::MISSING;
    }
    if ($factory->impure()::MISSING) { echo $factory->impure()::MISSING; echo $factory->impure()::MISSING; }
    if ($factory->maybe()::MISSING) { echo $factory->maybe()::MISSING; echo $factory->maybe()::MISSING; }
    if ($factory->pure()::OTHER) { echo $other->pure()::OTHER; }
}

function statics(): void {
    if (Factory::pureStatic()::MISSING) { echo Factory::pureStatic()::MISSING; }
    if (Factory::impureStatic()::MISSING) { echo Factory::impureStatic()::MISSING; }
    if (Factory::maybeStatic()::MISSING) { echo Factory::maybeStatic()::MISSING; }
}

function arguments(Factory $factory, int $index): void {
    if (pureReceiver($index)::MISSING) {
        echo pureReceiver($index)::MISSING;
        $index++;
        echo pureReceiver($index)::MISSING;
    }
    if (pureReceiver(0)::MISSING) {
        echo pureReceiver(0)::MISSING;
        echo pureReceiver(1)::MISSING;
        echo pureReceiver(index: 0)::MISSING;
    }
    if ($factory->pure(0)::MISSING) {
        echo $factory->pure(0)::MISSING;
        echo $factory->pure(1)::MISSING;
        $factory = new Factory;
        echo $factory->pure(0)::MISSING;
    }
}

function moreInvalidations(Factory $factory, int $index): void {
    if (pureReceiver($index)::REFERENCE) {
        replace($index);
        echo pureReceiver($index)::REFERENCE;
    }
    if ($factory->pure()::VOID_RETURN) {
        $factory->pureVoid();
        echo $factory->pure()::VOID_RETURN;
    }
    if ($factory->pure()::CONSTRUCTOR) {
        $factory->__construct();
        echo $factory->pure()::CONSTRUCTOR;
    }
    if ($factory->fluent()::FLUENT) {
        echo $factory->fluent()::FLUENT;
        echo $factory->fluent()::FLUENT;
    }
    if ($factory->pureFluent()::PURE_FLUENT) {
        echo $factory->pureFluent()::PURE_FLUENT;
        echo $factory->pureFluent()::PURE_FLUENT;
    }
}

function literalArguments(Factory $factory): void {
    if (value(true)::BOOL) { echo value(true)::BOOL; echo value(false)::BOOL; }
    if (value(null)::NULL_VALUE) { echo value(null)::NULL_VALUE; }
    if (value('$factory')::STRING_VALUE) {
        $factory = new Factory;
        echo value('$factory')::STRING_VALUE;
    }
}

function builtins(\ArrayObject $factory): void {
    if ($factory->getIterator()::BUILTIN) {
        echo $factory->getIterator()::BUILTIN;
        $factory->append(1);
        echo $factory->getIterator()::BUILTIN;
    }
}

function resolvedFunctionNames(): void {
    if (imported()::ALIASED) {
        echo \ConstantCallGuards\pureReceiver()::ALIASED;
        echo pureReceiver()::ALIASED;
    }
    if (pureReceiver()::FALLBACK) {
        echo \ConstantCallGuards\pureReceiver()::FALLBACK;
    }
    if (\ConstantCallGuards\pureReceiver()::QUALIFIED) {
        echo namespace\pureReceiver()::QUALIFIED;
    }
}

function referencesAndFluent(Factory $factory, int $index): void {
    if (pureReceiver($index)::METHOD_REFERENCE) {
        $factory->replace(index: $index);
        echo pureReceiver($index)::METHOD_REFERENCE;
    }
    if (pureReceiver($index)::STATIC_REFERENCE) {
        Factory::replaceStatic($index);
        echo pureReceiver($index)::STATIC_REFERENCE;
    }
    if ($factory->thisFluent()::THIS_FLUENT) {
        echo $factory->thisFluent()::THIS_FLUENT;
        echo $factory->thisFluent()::THIS_FLUENT;
    }
}

/** @param 'FIRST'|'SECOND' $name */
function finiteSelectors(Factory $factory, string $name): void {
    if ($factory->pure()::{$name}) {
        echo $factory->pure()::{$name};
        if ($name === 'FIRST') { echo $factory->pure()::{$name}; }
        $factory->mutate();
        echo $factory->pure()::{$name};
    }
}

function callBranches(bool $flag, Factory $factory): void {
    if ($flag) {
        if (!$factory->pure()::JOINED) { return; }
    } else {
        if (!$factory->pure()::JOINED) { return; }
    }
    echo $factory->pure()::JOINED;
    if ($flag) { if (!$factory->pure()::ONE_PATH) { return; } }
    echo $factory->pure()::ONE_PATH;
    if (impureReceiver()::COMPARISON === true) { echo impureReceiver()::COMPARISON; }
    if (true === impureReceiver()::REVERSED) { echo impureReceiver()::REVERSED; }
}

class Holder {
    public function __construct(public readonly Factory $factory) {}
    public function mutate(): void {}
}

function nestedReadonly(Holder $holder, Factory $factoryLong, Factory $factory): void {
    if ($holder->factory->pure()::READONLY_NESTED) {
        $holder->mutate();
        echo $holder->factory->pure()::READONLY_NESTED;
    }
    if ($factoryLong->pure()::LONG_NAME) {
        $factory->mutate();
        echo $factoryLong->pure()::LONG_NAME;
    }
}

/**
 * @param array{int} $indexes
 * @param array{int} $other
 */
function unpackedArguments(array $indexes, array $other): void {
    if (pureReceiver(...$indexes)::UNPACKED) {
        echo pureReceiver(...$indexes)::UNPACKED;
        echo pureReceiver(...$other)::UNPACKED;
        $indexes[0]++;
        echo pureReceiver(...$indexes)::UNPACKED;
    }
}

class ChildFactory extends Factory {
    public function lexicalParent(): void {
        if (Factory::pureStatic()::PARENT_NAME) {
            $this->mutate();
            echo Factory::pureStatic()::PARENT_NAME;
        }
        if (FACTORY::pureStatic()::PARENT_CASE) {
            $this->mutate();
            echo FACTORY::pureStatic()::PARENT_CASE;
        }
    }
}
