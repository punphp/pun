<?php
class NullsafeBoundarySink {
    public ?self $child = null;
    public function foo($value, $later = null, $last = null): ?self { return $this; }
    public function bar($value, $later = null, $last = null): ?self { return $this; }
    public function read(): ?self { return $this->child; }
}
class NullsafeBoundaryOther extends NullsafeBoundarySink {}
class NullsafeBoundaryInspect {
    public function inspect($value, $later = null, $last = null) {}
}
function nullsafeReceiver(?NullsafeBoundarySink $sink): ?NullsafeBoundarySink { return $sink; }
function nullsafeBoundaries(?NullsafeBoundarySink $sink, bool $condition): void {
    $inspect = new NullsafeBoundaryInspect();
    nullsafeReceiver($sink)?->foo($sink, [$sink]); // observe
    $sink?->foo($born = 1); // observe
    $inspect->inspect($sink, $born); // observe
    $sink = null;
    $sink?->foo($neverBorn = 2); // observe
    $inspect->inspect($sink, $neverBorn); // observe
    $sink = new NullsafeBoundarySink();
    $sink?->foo($definitelyBorn = 3); // observe
    $inspect->inspect($sink, $definitelyBorn); // observe
    $sink->child?->foo([$sink->child], $sink->child = null, [$sink->child]); // observe
    $inspect->inspect($sink->child); // observe
    $sink?->foo($sink?->foo(1)/* observe */, [$sink]); // observe
    $sink->read()?->foo([$sink->read()]); // observe
}
function nullsafeCorrelations(bool $condition): void {
    if ($condition) {
        $sink = new NullsafeBoundarySink();
        $name = 'foo';
        $value = 1;
    } else {
        $sink = new NullsafeBoundaryOther();
        $name = 'bar';
        $value = 'other';
    }
    $sink?->$name([$value], [$sink], $value); // observe
    $holders = ['sink' => $sink];
    $holders['sink']?->$name([$holders], $holders['sink']); // observe
}
/** @param array{sink: NullsafeBoundarySink|null} $holders */
function nullsafeOffsets(array $holders): void {
    $inspect = new NullsafeBoundaryInspect();
    $holders['sink']?->foo([$holders], $holders['sink'] = null, [$holders]); // observe
    $inspect->inspect($holders); // observe
}
function nullsafeReceiverAssignment(?NullsafeBoundarySink $maybe): void {
    $inspect = new NullsafeBoundaryInspect();
    ($sink = $maybe)?->foo($sink, [$sink]); // observe
    $inspect->inspect($sink); // observe
}
