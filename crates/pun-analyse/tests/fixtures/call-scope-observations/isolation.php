<?php
class IsolationSink {
    public function foo($x) {}
    public function bar($x) {}
}
function isolation(IsolationSink $o, bool $b): void {
    $m = $b ? 'foo' : 'bar';
    $o->{($v = $m)}((function () use ($v) {
        function observationLocal($x) { return $x; }
        class ObservationLocal {}
        interface ObservationLocalInterface {}
        trait ObservationLocalTrait {}
        enum ObservationLocalEnum { case A; }
        return $v;
    })()); // observe
    $o->{($v = $m)}((new class($v) {
        public function __construct($v) {}
        public function value($v): int { return 1; }
    })->value($v)); // observe
}
