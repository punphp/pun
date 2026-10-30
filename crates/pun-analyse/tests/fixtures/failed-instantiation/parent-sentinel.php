<?php
declare(strict_types=1);

namespace FailedInstantiationParentSentinel;

// These missing-parent expressions are parser-valid PHPStan inputs, but PHP's
// compiler rejects new parent outside a class or inside a parentless class.
final class Ordinary {
    public int $property = 7;
    public function method(): int { return 8; }
}

function acceptObject(object $value): void {}

function outside(bool $ordinary): void {
    $first = 0;
    $second = 0;
    $value = /*probe*/ new parent($first = 11, $second = 29, $first = 37);
    $_ = /*probe*/ $value;
    $_ = /*probe*/ $first;
    $_ = /*probe*/ $second;
    $copy = $value;
    $array = [$copy];
    $_ = /*probe*/ $copy;
    $_ = /*probe*/ $array;
    $_ = /*probe*/ $array[0];
    if ($ordinary) {
        $joined = new Ordinary();
    } else {
        $joined = $copy;
    }
    $_ = /*probe*/ $joined;
    /*probe*/ acceptObject($value);
    /*probe*/ acceptObject($joined);
    $control = /*probe*/ new Ordinary();
    /*probe*/ acceptObject($control);
    $_ = 'after-outside';
}

class Parentless {
    public function inside(): void {
        $value = /*probe*/ new parent($written = 41);
        $_ = /*probe*/ $value;
        $_ = /*probe*/ $written;
        $_ = 'after-inside';
    }
}

class UnresolvedChild extends MissingAncestor {
    public function inheritedMetadata(): void {
        $value = /*probe*/ new parent($written = 43);
        $_ = /*probe*/ $value;
        $_ = /*probe*/ $written;
        $_ = 'after-unresolved-parent';
    }
}

function projection(): void {
    $value = new parent();
    $name = /*probe*/ get_class($value);
    $literal = /*probe*/ $value::class;
    $_ = /*probe*/ $name;
    $_ = /*probe*/ $literal;
    $fromName = /*probe*/ new $name();
    $fromLiteral = /*probe*/ new $literal();
    $_ = /*probe*/ $fromName;
    $_ = /*probe*/ $fromLiteral;
    $_ = 'after-projection';
}

function dynamicSentinel(): void {
    $value = new parent();
    $first = 0;
    $second = 0;
    $dynamic = /*probe*/ new $value($first = 47, $second = 59, $first = 61);
    $_ = /*probe*/ $dynamic;
    $_ = /*probe*/ $first;
    $_ = /*probe*/ $second;
    $_ = 'after-dynamic';
}

// Keep each possibly never-producing operation in its own function.
function booleanCast(): void {
    $value = new parent();
    $result = /*probe*/ (bool) $value;
    $_ = /*probe*/ $result;
    $_ = 'after-bool';
}
function integerCast(): void {
    $value = new parent();
    $result = /*probe*/ (int) $value;
    $_ = /*probe*/ $result;
    $_ = 'after-int';
}
function floatCast(): void {
    $value = new parent();
    $result = /*probe*/ (float) $value;
    $_ = /*probe*/ $result;
    $_ = 'after-float';
}
function stringCast(): void {
    $value = new parent();
    $result = /*probe*/ (string) $value;
    $_ = /*probe*/ $result;
    $_ = 'after-string';
}
function arrayCast(): void {
    $value = new parent();
    $result = /*probe*/ (array) $value;
    $_ = /*probe*/ $result;
    $_ = 'after-array';
}
function objectCast(): void {
    $value = new parent();
    $result = /*probe*/ (object) $value;
    $_ = /*probe*/ $result;
    $_ = 'after-object';
}
function propertyValue(): void {
    $value = new parent();
    $result = /*probe*/ $value->property;
    $_ = /*probe*/ $result;
    $_ = 'after-property';
}
function methodValue(): void {
    $value = new parent();
    $result = /*probe*/ $value->method();
    $_ = /*probe*/ $result;
    $_ = 'after-method';
}
function offsetValue(): void {
    $value = new parent();
    $result = /*probe*/ $value[0];
    $_ = /*probe*/ $result;
    $_ = 'after-offset';
}
function cloneValue(): void {
    $value = new parent();
    $result = /*probe*/ clone $value;
    $_ = /*probe*/ $result;
    $_ = 'after-clone';
}
function foreachValue(): void {
    $value = new parent();
    foreach ($value as $key => $item) {
        $_ = /*probe*/ $key;
        $_ = /*probe*/ $item;
        $_ = 'after-foreach-body';
    }
    $_ = 'after-foreach';
}

/** @param class-string<Ordinary> $class */
function classStringControl(string $class): void {
    $_ = /*probe*/ $class;
    $literal = /*probe*/ Ordinary::class;
    $value = /*probe*/ new $class();
    $_ = /*probe*/ $value;
    $_ = /*probe*/ get_class($value);
    $_ = /*probe*/ $value::class;
    /*probe*/ acceptObject($value);
    $_ = 'after-class-string';
}

function staticMethodValue(): void {
    $value = new parent();
    $result = /*probe*/ $value::method();
    $_ = /*probe*/ $result;
    $_ = 'after-static-method';
}
function staticPropertyValue(): void {
    $value = new parent();
    $result = /*probe*/ $value::$property;
    $_ = /*probe*/ $result;
    $_ = 'after-static-property';
}
function constantValue(): void {
    $value = new parent();
    $result = /*probe*/ $value::VALUE;
    $_ = /*probe*/ $result;
    $_ = 'after-constant';
}
function unaryPlus(): void {
    $value = new parent();
    $result = /*probe*/ +$value;
    $_ = /*probe*/ $result;
    $_ = 'after-unary-plus';
}
function unaryMinus(): void {
    $value = new parent();
    $result = /*probe*/ -$value;
    $_ = /*probe*/ $result;
    $_ = 'after-unary-minus';
}
function bitwiseNot(): void {
    $value = new parent();
    $result = /*probe*/ ~$value;
    $_ = /*probe*/ $result;
    $_ = 'after-bitwise-not';
}

function unionBooleanCast(bool $ordinary): void {
    $value = $ordinary ? new Ordinary() : new parent();
    $_ = /*probe*/ $value;
    $result = /*probe*/ (bool) $value;
    $_ = /*probe*/ $result;
    $_ = 'after-union-bool';
}
function unionIntegerCast(bool $ordinary): void {
    $value = $ordinary ? new Ordinary() : new parent();
    $_ = /*probe*/ $value;
    $result = /*probe*/ (int) $value;
    $_ = /*probe*/ $result;
    $_ = 'after-union-int';
}
function unionFloatCast(bool $ordinary): void {
    $value = $ordinary ? new Ordinary() : new parent();
    $_ = /*probe*/ $value;
    $result = /*probe*/ (float) $value;
    $_ = /*probe*/ $result;
    $_ = 'after-union-float';
}
function unionStringCast(bool $ordinary): void {
    $value = $ordinary ? new Ordinary() : new parent();
    $_ = /*probe*/ $value;
    $result = /*probe*/ (string) $value;
    $_ = /*probe*/ $result;
    $_ = 'after-union-string';
}
function unionArrayCast(bool $ordinary): void {
    $value = $ordinary ? new Ordinary() : new parent();
    $_ = /*probe*/ $value;
    $result = /*probe*/ (array) $value;
    $_ = /*probe*/ $result;
    $_ = 'after-union-array';
}
function unionObjectCast(bool $ordinary): void {
    $value = $ordinary ? new Ordinary() : new parent();
    $_ = /*probe*/ $value;
    $result = /*probe*/ (object) $value;
    $_ = /*probe*/ $result;
    $_ = 'after-union-object';
}
function unionForeach(bool $array): void {
    $value = $array ? ['key' => 71, 2 => 73] : new parent();
    $_ = /*probe*/ $value;
    foreach ($value as $key => $item) {
        $_ = /*probe*/ $key;
        $_ = /*probe*/ $item;
        $_ = 'after-union-foreach-body';
    }
    $_ = 'after-union-foreach';
    foreach (['key' => 71, 2 => 73] as $key => $item) {
        $_ = /*probe*/ $key;
        $_ = /*probe*/ $item;
        $_ = 'after-array-control-body';
    }
    $_ = 'after-array-control';
}

final class StaticOrdinary {
    public static int $property = 13;
    public const int VALUE = 17;
    public static function method(): int { return 19; }
}
function staticAndUnaryControls(): void {
    $value = new StaticOrdinary();
    $_ = /*probe*/ $value::method();
    $_ = /*probe*/ $value::$property;
    $_ = /*probe*/ $value::VALUE;
    $_ = /*probe*/ +7;
    $_ = /*probe*/ -7;
    $_ = /*probe*/ ~7;
    $_ = 'after-static-unary-controls';
}
