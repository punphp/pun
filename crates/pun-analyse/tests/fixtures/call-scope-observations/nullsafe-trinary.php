<?php
class NullsafeTrinarySink {
    public function foo($value) {}
    public function inspect($value) {}
}
function nullsafeMixedReceiver(mixed $sink): void {
    $sink?->foo($born = 1); // observe
    $inspect = new NullsafeTrinarySink();
    $inspect->inspect($born); // observe
}
function nullsafeVoidReceiver(): void {}
function nullsafeVoidResult(): void {
    nullsafeVoidReceiver()?->foo($born = 2); // observe
    $inspect = new NullsafeTrinarySink();
    $inspect->inspect($born); // observe
}
/** @template TSink of (NullsafeTrinarySink|null)
 * @param TSink $sink
 */
function nullsafeTemplateReceiver($sink): void {
    $sink?->foo($born = 3);
    $inspect = new NullsafeTrinarySink();
    $inspect->inspect($born); // observe
}
