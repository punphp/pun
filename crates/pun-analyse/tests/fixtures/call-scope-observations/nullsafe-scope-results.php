<?php
class nullsaferesultsink {
    public function value(): int { return 1; }
    public function inspect($value): void {}
}
function nullsafeScopeResults(?nullsaferesultsink $sink): void {
    $inspect = new nullsaferesultsink();
    $inspect->inspect($sink?->value()); // observe
    $sink = new nullsaferesultsink();
    $inspect->inspect($sink?->value()); // observe
    $sink = null;
    $inspect->inspect($sink?->value()); // observe
    $inspect->inspect(($sink = new nullsaferesultsink())?->value()); // observe
}
