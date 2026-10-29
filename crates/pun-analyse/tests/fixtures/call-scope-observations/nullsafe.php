<?php
class NullsafeObservationSink {
    public function foo($value, $later = null, $last = null) {}
    public function bar($value, $later = null, $last = null) {}
}
function nullsafeObservations(?NullsafeObservationSink $sink, bool $condition): void {
    if ($condition) {
        $name = 'foo';
        $value = 1;
    } else {
        $name = 'bar';
        $value = 'other';
    }
    $sink?->$name([$value], [$sink]); // observe
    $sink?->foo([$sink], $sink = null, [$sink]); // observe
    ($sink = new NullsafeObservationSink())?->foo($sink, [$sink]); // observe
}
