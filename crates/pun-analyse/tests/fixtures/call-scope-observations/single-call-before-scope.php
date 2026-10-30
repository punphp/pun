<?php
class BeforeScopeTargets {
    /**
     * @template T
     * @param T $later
     * @return T
     */
    public function foo($input, $later) { return $later; }
    /**
     * @template T
     * @param T $input
     * @return array{T}
     */
    public function bar($input, $later): array { return [$input]; }
}
function changingOperands(BeforeScopeTargets $target, bool $condition): void {
    $name = $condition ? 'foo' : 'bar';
    $i = 0;
    $target->$name(++$i, $i); // observe
}
function oneDynamicName(BeforeScopeTargets $target): void {
    $name = 'foo';
    $i = 10;
    $target->$name(++$i, $i); // observe
}
