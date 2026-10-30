<?php
class NestedSink {
    public function foo($x, $y = null): int { return 1; }
    public function bar($x, $y = null): int { return 1; }
}
function nested(NestedSink $o, NestedSink $inner, bool $b): void {
    if ($b) { $m = 'foo'; $v = 1; }
    else { $m = 'bar'; $v = 's'; }
    $n = 'foo';
    $o->$m($inner->$n($v)/* observe */, [$v]); // observe
}
