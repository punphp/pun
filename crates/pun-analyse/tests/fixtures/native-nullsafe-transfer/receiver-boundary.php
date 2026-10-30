<?php

// The former nullsafe-only unavailable case from native_properties.rs.
class Constructor { public function __construct() {} }
/** @template T */ class Generic {}
class Simple {}
/**
 * @property mixed $magic
 * @property int $annotated
 */
class NativeBoundary {
    public mixed $prop;
    public mixed $annotated;
    public array $items;
    public mixed $hooked { get => null; }
    public static mixed $staticProp;
    public function arrayReturn(): array { return []; }
    public function required(int $value): mixed { return $value; }
    public function &reference(): mixed { return $this->prop; }
    public function example(string $selector, bool $flag): void {
        $copy = $this->prop;
        $alias =& $copy;
        $callable = fn () => $this->prop;
        $this->prop; // observe
        $this?->prop; // observe
        $this->unavailable; // unavailable
    }
}
