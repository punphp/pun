<?php
class NumberOutput {
    /** @param-out int $value */
    public function output(&$value): void {}
}
class StringOutput {
    /** @param-out string $value */
    public function output(&$value): void {}
}
function unionOutput(NumberOutput|StringOutput $target): void {
    $value = 0;
    $target->output($value); // observe
    $later = $value;
}
class DynamicOutput {
    /** @param-out int $value */
    public function foo(&$value): void {}
    /** @param-out string $value */
    public function bar(&$value): void {}
}
function nameOutput(DynamicOutput $target, bool $condition): void {
    $value = 0;
    $name = $condition ? 'foo' : 'bar';
    $target->$name($value); // observe
    $later = $value;
}
