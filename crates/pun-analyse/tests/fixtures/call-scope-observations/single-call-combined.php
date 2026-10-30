<?php
class OptionalOutput {
    /** @param-out int $value */
    public function output(&$value, $extra = null): void {}
}
class MissingOutput {
    public function output(string &$value): void {}
}
function incompleteOut(OptionalOutput|MissingOutput $target): void {
    $value = 'before';
    $target->output($value); // observe
    $after = $value;
}
class TailLeft {
    /** @param callable(int): int $callback */
    public function invoke($callback, string ...$tail): int { return 1; }
}
class TailRight {
    /** @param callable(string): string $callback */
    public function invoke($callback, int $first, bool $second): string { return 'right'; }
}
function tailSwallowsFixed(TailLeft|TailRight $target): void {
    $i = 0;
    $target->invoke(fn($value) => $value, ++$i, $i); // observe
}
class NameLeft {
    /** @param-out int $left */
    public function output(&$left): void {}
}
class NameRight {
    /** @param-out string $right */
    public function output(&$right): void {}
}
function differingNames(NameLeft|NameRight $target): void {
    $value = 0;
    $target->output(left: $value); // observe
    $after = $value;
}
class ThisInteger {
    public function read(): int { return 1; }
}
class ThisString {
    public function read(): string { return 'string'; }
}
class BindLeft {
    /**
     * @param \Closure(): mixed $callback
     * @param-closure-this ThisInteger $callback
     */
    public function invoke($callback): int { return 1; }
}
class BindRight {
    /**
     * @param \Closure(): mixed $callback
     * @param-closure-this ThisString $callback
     */
    public function invoke($callback): string { return 'right'; }
}
function combinedThis(BindLeft|BindRight $target): void {
    $target->invoke(function () { return $this->read(); }); // observe
}
class BindAbsent {
    /** @param \Closure(): mixed $callback */
    public function invoke($callback): string { return 'absent'; }
}
function incompleteThis(BindLeft|BindAbsent $target): void {
    $target->invoke(function () { return $this->read(); }); // observe
}
