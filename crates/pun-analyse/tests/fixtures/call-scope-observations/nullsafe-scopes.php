<?php
class nullsafescopesink {
    public ?self $child = null;
    public function foo($value, $later = null, $last = null): ?self { return $this; }
    public function inspect($value, $later = null, $last = null) {}
}
/** @param array{nest: array{sink: nullsafescopesink|null}, other: 0} $grid */
function nullsafeScopeNested(array $grid): void {
    $inspect = new nullsafescopesink();
    $grid['nest']['sink']?->foo($grid['other'] = 1);
    $inspect->inspect($grid, $grid['nest'], $grid['nest']['sink']); // observe
}
function nullsafeScopeVariable(?nullsafescopesink $sink): void {
    $inspect = new nullsafescopesink();
    $sink?->foo($sink = null, $born = 1);
    $inspect->inspect($sink, $born); // observe
    $sink = null;
    $sink?->foo($neverBorn = 2);
    $inspect->inspect($sink, $neverBorn); // observe
    $sink = new nullsafescopesink();
    $sink?->foo($definitelyBorn = 3);
    $inspect->inspect($sink, $definitelyBorn); // observe
}
function nullsafeScopeProperty(nullsafescopesink $sink): void {
    $inspect = new nullsafescopesink();
    $sink->child?->foo($sink->child = null);
    $inspect->inspect($sink->child); // observe
}
function nullsafeScopeAssignment(?nullsafescopesink $maybe): void {
    $inspect = new nullsafescopesink();
    ($sink = $maybe)?->foo($sink = null);
    $inspect->inspect($sink); // observe
    $sink = new nullsafescopesink();
    ($sink = null)?->foo($born = 1);
    $inspect->inspect($sink, $born); // observe
}
function nullsafeScopeMixed(mixed $sink): void {
    $inspect = new nullsafescopesink();
    $sink?->foo($born = 1);
    $inspect->inspect($born); // observe
}
/** @param array{sink: nullsafescopesink|null} $holders */
function nullsafeScopeOffset(array $holders): void {
    $inspect = new nullsafescopesink();
    $holders['sink']?->foo($holders['sink'] = null);
    $inspect->inspect($holders, $holders['sink']); // observe
}
function nullsafeScopeCalls(?nullsafescopesink $sink): void {
    $inspect = new nullsafescopesink();
    nullsafeScopeReceiver($sink)?->foo($born = 1);
    $inspect->inspect($sink, $born); // observe
}
function nullsafeScopeReceiver(?nullsafescopesink $sink): ?nullsafescopesink { return $sink; }
