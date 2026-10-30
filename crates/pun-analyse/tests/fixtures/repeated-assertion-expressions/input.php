<?php

namespace RepeatedAssertionExpressions;

use PHPUnit\Framework\TestCase;

class Value {}
class Factory
{
    public ?Value $value;

    /** @phpstan-pure */
    public function make(): ?Value { return $this->value; }

    public function mutate(): void { $this->value = null; }
}

function mutate(Factory $factory): void { $factory->mutate(); }

class Assertions extends TestCase
{
    public function direct(Factory $factory): void
    {
        $this->assertNotNull($factory->make());
        $factory->make(); // observe
    }

    public function impossible(Factory $factory): void
    {
        $this->assertTrue('5' === $factory->make());
        $factory->make(); // observe
    }

    public function impossibleAfterFail(Factory $factory): void
    {
        $this->fail();
        $this->assertTrue('5' === $factory->make());
        $factory->make(); // observe
    }

    public function impossibleAfterReturn(Factory $factory): void
    {
        return;
        $this->assertTrue('5' === $factory->make());
        $factory->make(); // observe
    }

    public function aliases(Factory $factory): void
    {
        $alias = $factory;
        $this->assertNotNull($alias->make());
        $alias->make(); // observe
        $factory->make(); // observe
    }

    public function changedReceiver(Factory $factory, Factory $other): void
    {
        $this->assertNotNull($factory->make());
        $factory = $other;
        $factory->make(); // observe
    }

    public function impureReceiver(Factory $factory): void
    {
        $this->assertNotNull($factory->make());
        $factory->mutate();
        $factory->make(); // observe
    }

    public function impureFunction(Factory $factory): void
    {
        $this->assertNotNull($factory->make());
        mutate($factory);
        $factory->make(); // observe
    }

    public function unrelatedReceiver(Factory $factory, Factory $other): void
    {
        $this->assertNotNull($factory->make());
        $other->mutate();
        $factory->make(); // observe
    }

    public function positive(Factory $factory): void
    {
        $this->assertTrue($factory->make() !== null);
        $factory->make(); // observe
    }

    public function negative(Factory $factory): void
    {
        $this->assertFalse($factory->make() !== null);
        $factory->make(); // observe
    }

    public function unrelatedCall(Factory $factory, Factory $other): void
    {
        $this->assertNotNull($factory->make());
        $other->make();
        $factory->make(); // observe
    }

    public function unspecifiedPurity(LooseFactory $factory): void
    {
        $this->assertNotNull($factory->make());
        $factory->make(); // observe
    }

    public function unspecifiedImpossible(LooseFactory $factory): void
    {
        $this->assertTrue('5' === $factory->make());
        $factory->make(); // observe
    }

    public function explicitlyImpure(LooseFactory $factory): void
    {
        $this->assertNotNull($factory->impure());
        $factory->impure(); // observe
    }

    public function unspecifiedAfterMutation(LooseFactory $factory): void
    {
        $this->assertNotNull($factory->make());
        $factory->mutate();
        $factory->make(); // observe
    }

    public function unspecifiedAfterUnrelatedMutation(LooseFactory $factory, LooseFactory $other): void
    {
        $this->assertNotNull($factory->make());
        $other->mutate();
        $factory->make(); // observe
    }

    public function unspecifiedAfterOtherMethod(LooseFactory $factory): void
    {
        $this->assertNotNull($factory->make());
        $factory->other();
        $factory->make(); // observe
    }
}

class LooseFactory
{
    public ?Value $value;
    public function make(): ?Value { return $this->value; }
    /** @phpstan-impure */
    public function impure(): ?Value { return $this->value; }
    public function mutate(): void { $this->value = null; }
    public function other(): ?Value { return $this->value; }
}

class TailAssertions extends TestCase
{
    public function afterFail(LooseFactory $factory): void
    {
        $this->fail();
        $this->assertTrue('5' === $factory->make());
        $factory->make(); // observe
    }

    public function afterReturn(LooseFactory $factory): void
    {
        return;
        $this->assertTrue('5' === $factory->make());
        $factory->make(); // observe
    }

    public function changedReceiver(LooseFactory $factory, LooseFactory $other): void
    {
        $this->assertNotNull($factory->make());
        $factory = $other;
        $factory->make(); // observe
    }

    public function aliasMutation(LooseFactory $factory): void
    {
        $alias = $factory;
        $this->assertNotNull($factory->make());
        $alias->mutate();
        $factory->make(); // observe
    }

    public function declaredMethodAssertion(Factory $factory): void
    {
        $this->hasValue($factory);
        $factory->make(); // observe
    }

    /** @phpstan-assert Value $factory->make() */
    private function hasValue(Factory $factory): void {}
}

class ScalarAssertions extends TestCase
{
    public function scalarComparison($a): void
    {
        $this->assertSame('5', $a);
        $this->assertEquals('5', $a);
        $this->assertTrue($a === $this->value());
        $this->value(); // observe
        $mock = $this->createMock(\stdClass::class);
        $this->value(); // observe
        $mock->method('x')->with($this->callback(function ($x) {
            return true;
        }))->willReturn($this->value());
        $this->value(); // observe
    }

    public function looseComparison($a): void
    {
        $this->assertSame('5', $a);
        $this->assertTrue($a == $this->value());
        $this->value(); // observe
    }

    private function value(): int { return 1; }
}

class StaticFactory
{
    public static ?Value $value;
    public static function make(): ?Value { return self::$value; }
    /** @phpstan-impure */
    public static function impure(): ?Value { return self::$value; }
    /** @phpstan-impure */
    public static function mutate(): void { self::$value = null; }
}

class StaticAssertions extends TestCase
{
    public function unspecifiedStatic(StaticFactory $factory): void
    {
        $this->assertNotNull($factory->make());
        $factory->make(); // observe
    }

    public function impureStatic(StaticFactory $factory): void
    {
        $this->assertNotNull($factory->impure());
        $factory->impure(); // observe
    }

    public function mutatedStatic(StaticFactory $factory): void
    {
        $this->assertNotNull($factory->make());
        $factory->mutate();
        $factory->make(); // observe
    }
}
