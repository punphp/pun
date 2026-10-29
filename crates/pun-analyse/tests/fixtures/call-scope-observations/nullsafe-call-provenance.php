<?php
/** @template T of (stdClass|null) */
class NullsafeCallProvenance {
    public function voidOnly(): void {}
    public function nullOnly(): null { return null; }
    /** @return T */
    public function templateBound() { return null; }
    public function consume($value): void {}
}
function genuineNullReturn(?NullsafeCallProvenance $sink, bool $condition): void {
    $inspect = new NullsafeCallProvenance();
    $name = $condition ? 'voidOnly' : 'nullOnly';
    $inspect->consume($sink?->$name()); // observe
    $inspect->consume($sink?->voidOnly()); // observe
    $inspect->consume($sink?->nullOnly()); // observe
}
/**
 * @template T of (stdClass|null)
 * @param NullsafeCallProvenance<T>|null $sink
 */
function genericNullReturn($sink): void {
    $inspect = new NullsafeCallProvenance();
    $inspect->consume($sink?->templateBound()); // observe
}
