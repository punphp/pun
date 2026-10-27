<?php
namespace CloneControls;
class Item {}
interface Tag {}
class Child extends Item implements Tag {}
enum Choice { case One; }
function declared(mixed $m): void { $r = fn () => clone $m; }
function implicit($m): void { clone $m; }
/** @param mixed $m */
function documented($m): void { clone $m; }
function anyObject(object $m): void { clone $m; }
function named(Item $m): void { clone $m; }
function unionScalar(Item|int $m): void { clone $m; }
function unionObject(object|string $m): void { clone $m; }
function nullable(?Item $m): void { clone $m; }
function scalar(int $m): void { clone $m; }
function literal(): void { clone 1; }
function arrayValue(array $m): void { clone $m; }
function enumeration(Choice $m): void { clone $m; }
function closure(\Closure $m): void { clone $m; }
function callableValue(callable $m): void { clone $m; }
/** @param callable(int): string $m */
function typedCallable($m): void { clone $m; }
function intersection(Item&Tag $m): void { clone $m; }
/**
 * @template T
 * @param T $m
 */
function template($m): void { clone $m; }
/**
 * @template T of object
 * @param T $m
 */
function objectTemplate($m): void { clone $m; }
/**
 * @template T of Item
 * @param T $m
 */
function namedTemplate($m): void { clone $m; }
/**
 * @template T of Item|int
 * @param T $m
 */
function unionTemplate($m): void { clone $m; }
/**
 * @template T of int
 * @param T $m
 */
function scalarTemplate($m): void { clone $m; }
class Owner {
    function copy(): void { clone $this; }
    function unionThis(Item $m, bool $replace): void {
        if ($replace) { $m = $this; }
        clone $m;
    }
}
function evaluate(mixed $m): void {
    clone ($m = new Item());
    $m; // observe
}
function unknown(): void { clone missing_operand(); }
function notObject(mixed $m): void { if (!is_object($m)) { clone $m; } }
function terminate(): never { throw new \Exception(); }
function neverValue(): void { clone terminate(); }
