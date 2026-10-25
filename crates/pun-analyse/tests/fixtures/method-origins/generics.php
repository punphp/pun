<?php

namespace MethodOriginsGenerics;

use function PHPStan\dumpType;
use function PHPStan\dumpPhpDocType;

/** @template T */
abstract class Box
{
    /** @return T */
    abstract public function value();

    /** @param callable(T): void $transform */
    public function each(callable $transform): void {}

    public function showThis(): void { dumpType($this->value()); }

    /** @return array<T> */
    public static function values(): array { return []; }

    public function showStatic(): void { dumpType(self::values(), static::values()); }
}

/**
 * @template U
 * @extends Box<U>
 */
abstract class ChildBox extends Box {}

/** @extends Box<array{z: string, a: bool}> */
abstract class ShapeBox extends Box {}

/** @param Box<array{z: string, a: bool}> $box */
function direct(Box $box): void { dumpType($box->value()); }

/** @param ChildBox<array{z: string, a: bool}> $child */
function inherited(ChildBox $child): void { dumpPhpDocType($child->value()); }

function fixedAncestor(ShapeBox $shape): void { dumpType($shape->value()); }

/** @param Box<list<mixed>> $box */
function explicit(Box $box): void { dumpType($box->value()); }

/** @param Box<array> $box */
function implicit(Box $box): void { dumpType($box->value()); }

/** @param Box<array{z: string, a: bool}> $box */
function directCallback(Box $box): void {
    $box->each(transform: static function (array $row): void { dumpType($row); });
}

/** @param ChildBox<array{z: string, a: bool}> $child */
function inheritedCallback(ChildBox $child): void {
    $child->each(static fn(array $row) => dumpPhpDocType($row));
}

function fixedAncestorCallback(ShapeBox $shape): void {
    $shape->each(static function (array $row): void { dumpType($row); });
}

/**
 * @template T of Box<array{z: string, a: bool}>
 * @param T $box
 */
function boundedReceiver($box): void { dumpType($box->value()); }

class Mapper
{
    /**
     * @template V
     * @template RK of array-key
     * @template RV
     * @param array<V> $source
     * @param callable(V): array{RK, RV} $transform
     * @return array<RK, RV>
     */
    public static function pairs(array $source, callable $transform): array { return []; }
}

/** @param array<array{z: string, a: bool}> $rows */
function callbacks(Mapper $mapper, array $rows): void
{
    dumpType(Mapper::pairs(transform: static function (array $row): array {
        dumpType($row);
        return [$row['z'], $row['a']];
    }, source: $rows));
    $captured = 29;
    dumpType($mapper->pairs([1, 2], static fn(int $value) => [17, $captured]));
    dumpType($mapper->pairs([1, 2], static fn(int $value) => [19, 'literal']));
}
