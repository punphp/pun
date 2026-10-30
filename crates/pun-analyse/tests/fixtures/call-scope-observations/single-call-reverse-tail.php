<?php
class ReverseFixed {
    /** @param callable(int): int $callback */
    public function invoke($callback, int $first, bool $second): int { return 1; }
}
class ReverseVariadic {
    /** @param callable(string): string $callback */
    public function invoke($callback, string ...$tail): string { return 'tail'; }
}
function fixedBeforeVariadic(ReverseFixed|ReverseVariadic $target): void {
    $i = 0;
    $target->invoke(fn($value) => $value, ++$i, $i); // observe
}
