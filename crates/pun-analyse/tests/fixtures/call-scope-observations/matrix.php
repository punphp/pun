<?php
class ObservationSink {
    public function foo($x = null, $y = null) {}
    public function bar($x = null, $y = null) {}
    public function fixed($x = null, $y = null) {}
}
class ObservationOther extends ObservationSink {}
class ObservationBox {
    public ?int $value;
    /** @phpstan-impure */
    public function mutate(): int { $this->value = null; return 1; }
    /** @phpstan-pure */
    public function pure(): int { return 1; }
    /** @phpstan-impure
     * @return 'foo'|'bar'
     */
    public function choose(): string { $this->value = null; return rand() ? 'foo' : 'bar'; }
}
function correlated(bool $b): void {
    if ($b) { $m = 'foo'; $v = 1; $o = new ObservationSink; $args = ['x' => 1]; }
    else { $m = 'bar'; $v = 's'; $o = new ObservationOther; $args = ['x' => 's']; }
    $o->$m($v, [$v]); // observe
    $o->$m(...$args); // observe
}
function offset(ObservationSink $o, bool $b): void {
    $pair = $b ? ['m' => 'foo', 'v' => 1] : ['m' => 'bar', 'v' => 's'];
    $o->{$pair['m']}($pair['v'], [$pair['v']]); // observe
}
function dynamic(ObservationSink $o, bool $b): void {
    $v = $b ? 1 : 's';
    $o->{($b ? 'foo' : 'bar')}($v, [$v]); // observe
}
function writes(ObservationSink $o, bool $b): void {
    $m = $b ? 'foo' : 'bar';
    $v = 'old';
    $o->$m($v = 2, $v); // observe
    $v = 'old';
    $o->$m($v = 3, [$v]); // observe
    $v = 'old';
    $o->fixed($v = 4, [$v]); // observe
    $args = ['y' => 'old'];
    $o->$m($args = ['y' => 1], ...$args); // observe
    $o->$m($o = new ObservationOther, $o); // observe
    ($o = new ObservationSink)->$m($o, [$o]); // observe
}
function selector(ObservationSink $o, bool $b): void {
    $v = 'old';
    $o->{($v = $b ? 'foo' : 'bar')}($v, [$v]); // observe
    $m = 'foo';
    $objects = ['bar' => $o];
    $objects[$m = 'bar']->$m($m, [$m]); // observe
}
function impurity(ObservationSink $o, ObservationBox $box, bool $b): void {
    $m = $b ? 'foo' : 'bar';
    if ($box->value !== null) {
        $o->$m($box->mutate(), $box->value); // observe
    }
    if ($box->value !== null) {
        $o->$m($box->pure(), $box->value); // observe
    }
    if ($box->value !== null) {
        $o->{$box->choose()}($box->value, [$box->value]); // observe
    }
}
class ObservationTyped {
    public function foo(int $x, int $y): void {}
    public function bar(int $x, int $y): void {}
    public function fixed(int $x, int $y): void {}
}
function nativeDiagnostic(ObservationTyped $o, bool $b): void {
    $m = $b ? 'foo' : 'bar';
    $v = 'old';
    $o->$m($v = 2, $v); // observe
    $v = 'old';
    $o->fixed($v = 2, $v); // observe
}
