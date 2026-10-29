<?php
class IntegerCallback {
    /** @param callable(int): int $callback */
    public function invoke($callback): int { return $callback(1); }
}
class StringCallback {
    /** @param callable(string): string $callback */
    public function invoke($callback): string { return $callback('value'); }
}
function unionCallback(IntegerCallback|StringCallback $target): void {
    $target->invoke(fn($value) => $value); // observe
}
class GenericCallbackLeft {
    /**
     * @template T
     * @param T $input
     * @param callable(T): T $callback
     * @return T
     */
    public function invoke($input, $callback) { return $callback($input); }
}
class GenericCallbackRight {
    /**
     * @template T
     * @param T $input
     * @param callable(T): T $callback
     * @return array{T}
     */
    public function invoke($input, $callback): array { return [$callback($input)]; }
}
function genericCallback(GenericCallbackLeft|GenericCallbackRight $target): void {
    $i = 0;
    $target->invoke(++$i, fn($value) => $value); // observe
}
class NamedCallback {
    /** @param callable(int): int $callback */
    public function foo($callback): int { return $callback(1); }
    /** @param callable(string): string $callback */
    public function bar($callback): string { return $callback('value'); }
}
function dynamicCallback(NamedCallback $target, bool $condition): void {
    $name = $condition ? 'foo' : 'bar';
    $target->$name(fn($value) => $value); // observe
}
