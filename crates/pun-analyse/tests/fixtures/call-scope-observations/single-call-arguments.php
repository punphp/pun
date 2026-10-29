<?php
class SingleCallTargets {
    /**
     * @template T
     * @param T $input
     * @return T
     */
    public function foo($input, $later) { return $input; }
    /**
     * @template T
     * @param T $input
     * @return array{T, T}
     */
    public function bar($input, $later) { return [$input, $input]; }
}
function positional(SingleCallTargets $target, bool $condition): void {
    $name = $condition ? 'foo' : 'bar';
    $i = 0;
    $target->$name(++$i, $i); // observe
}
function named(SingleCallTargets $target, bool $condition): void {
    $name = $condition ? 'bar' : 'foo';
    $i = 10;
    $target->$name(input: ++$i, later: $i); // observe
}
class SingleCallUnpackTargets {
    public function foo($input, $later): int { return 1; }
    public function bar($later, $input): string { return 'bar'; }
}
function unpacked(SingleCallUnpackTargets $target, bool $condition): void {
    $name = $condition ? 'foo' : 'bar';
    $i = 20;
    $target->$name(...['input' => ++$i, 'later' => $i]); // observe
}
