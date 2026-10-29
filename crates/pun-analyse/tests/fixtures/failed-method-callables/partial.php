<?php
// Partial application is a separate Mago extension, not a PHPStan first-class callable.
class PartialTarget {
    public function known(string $value, int $number): int { return $number; }
}
function validPartial(PartialTarget $receiver): void {
    $closure = $receiver->known(?, 1);
    $post = $closure;
}
function invalidPartial(): void {
    $receiver = 'hello';
    $closure = $receiver->missing(?);
}
function invalidDynamicPartial(PartialTarget $receiver): void {
    $name = 123;
    $closure = $receiver->{$name}(?);
}
