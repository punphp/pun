<?php

namespace ConstantDynamicMethodGuards;

class Receiver {
    private const PRIVATE_NAME = 1;
}

class Factory {
    public const METHOD = 'pure';
    public const STATIC_METHOD = 'pureStatic';

    /** @phpstan-pure */
    public function pure(int $index = 0): Receiver { return new Receiver; }
    /** @phpstan-pure */
    public function other(): Receiver { return new Receiver; }
    /** @phpstan-impure */
    public function impure(): Receiver { echo 'changed'; return new Receiver; }
    /** @phpstan-pure */
    public static function pureStatic(int $index = 0): Receiver { return new Receiver; }
    /** @phpstan-pure */
    public static function otherStatic(): Receiver { return new Receiver; }
    public function mutate(): void {}
}

class OtherFactory extends Factory {}

function sink(mixed $value): void {}

function instanceLiterals(Factory $factory): void {
    if ($factory->{'pure'}()::LITERAL) {
        sink($factory->pure()::LITERAL);
        sink($factory->{'pure'}()::LITERAL);
        sink($factory->pure()::LITERAL);
    }
    if ($factory->{'impure'}()::IMPURE) {
        sink($factory->impure()::IMPURE);
        sink($factory->{'impure'}()::IMPURE);
    }
    if ($factory->{'pure'}()::CASE_NAME) {
        sink($factory->PURE()::CASE_NAME);
        sink($factory->pure()::CASE_NAME);
    }
    if ($factory->{'pure'}()::PRIVATE_NAME) {
        sink($factory->pure()::PRIVATE_NAME);
    }
}

function instanceVariables(Factory $factory): void {
    $method = 'pure';
    if ($factory->$method()::VARIABLE) {
        sink($factory->{$method}()::VARIABLE);
        sink($factory->pure()::VARIABLE);
        sink($factory->$method()::VARIABLE);
    }
    if ($factory->{$method}()::REVERSE) {
        sink($factory->$method()::REVERSE);
    }
    if ($factory->$method()::SELECTOR_WRITE) {
        $method = 'other';
        sink($factory->$method()::SELECTOR_WRITE);
    }
    if ($factory->$method()::RECEIVER_WRITE) {
        $factory = new Factory;
        sink($factory->$method()::RECEIVER_WRITE);
    }
}

function instanceExpressions(Factory $factory, int $index): void {
    if ($factory->{Factory::METHOD}()::CONSTANT_SELECTOR) {
        sink($factory->{Factory::METHOD}()::CONSTANT_SELECTOR);
        sink($factory->{'pure'}()::CONSTANT_SELECTOR);
    }
    $methods = ['pure'];
    if ($factory->{$methods[0]}()::ARRAY_SELECTOR) {
        sink($factory->{$methods[0]}()::ARRAY_SELECTOR);
        $methods[0] = 'other';
        sink($factory->{$methods[0]}()::ARRAY_SELECTOR);
    }
    if ($factory->{'pure'}($index)::ARGUMENT_WRITE) {
        $index++;
        sink($factory->{'pure'}($index)::ARGUMENT_WRITE);
    }
    if ($factory->{'pure'}(0)::ARGUMENT_SYNTAX) {
        sink($factory->pure(0)::ARGUMENT_SYNTAX);
        sink($factory->pure(index: 0)::ARGUMENT_SYNTAX);
    }
}

function staticLiterals(): void {
    if (Factory::{'pureStatic'}()::STATIC_LITERAL) {
        sink(Factory::{'pureStatic'}()::STATIC_LITERAL);
        sink(Factory::pureStatic()::STATIC_LITERAL);
    }
    if (Factory::pureStatic()::STATIC_REVERSE) {
        sink(Factory::{'pureStatic'}()::STATIC_REVERSE);
    }
    if (Factory::{'pureStatic'}()::STATIC_CASE) {
        sink(Factory::{'PURESTATIC'}()::STATIC_CASE);
    }
}

function staticVariables(): void {
    $method = 'pureStatic';
    if (Factory::$method()::STATIC_VARIABLE) {
        sink(Factory::{$method}()::STATIC_VARIABLE);
    }
    if (Factory::{$method}()::STATIC_REVERSE_VARIABLE) {
        sink(Factory::$method()::STATIC_REVERSE_VARIABLE);
    }
    if (Factory::{$method}()::STATIC_SELECTOR_WRITE) {
        $method = 'otherStatic';
        sink(Factory::$method()::STATIC_SELECTOR_WRITE);
    }
    if (Factory::{Factory::STATIC_METHOD}()::STATIC_CONSTANT_SELECTOR) {
        sink(Factory::{Factory::STATIC_METHOD}()::STATIC_CONSTANT_SELECTOR);
        sink(Factory::pureStatic()::STATIC_CONSTANT_SELECTOR);
    }
    $class = Factory::class;
    if ($class::$method()::STATIC_RECEIVER_WRITE) {
        $class = OtherFactory::class;
        sink($class::$method()::STATIC_RECEIVER_WRITE);
    }
}

function outgoingPaths(Factory $factory, bool $condition): void {
    $method = 'pureStatic';
    if ($condition) {
        if (Factory::$method()::ONE_PATH) {}
    }
    sink(Factory::$method()::ONE_PATH);
    if ($condition) {
        if (Factory::$method()::BOTH_PATHS) {}
    } else {
        if (Factory::{$method}()::BOTH_PATHS) {}
    }
    sink(Factory::$method()::BOTH_PATHS);
    if ($factory->{'pure'}()::OTHER_RECEIVER) {
        $factoryLong = new Factory;
        $factoryLong->mutate();
        sink($factory->pure()::OTHER_RECEIVER);
        $factory->mutate();
        sink($factory->pure()::OTHER_RECEIVER);
    }
}
