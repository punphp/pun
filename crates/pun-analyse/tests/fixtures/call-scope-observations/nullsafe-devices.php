<?php
class NullsafeDeviceSink {
    public ?self $child = null;
    public function foo($value, $later = null, $last = null): ?self { return $this; }
    public function bar($value, $later = null, $last = null): ?self { return $this; }
}
class NullsafeDeviceBox {
    /** @var array{sink: NullsafeDeviceSink|null} */
    public array $holders;
}
class NullsafeDeviceInspect {
    public function inspect($value, $later = null, $last = null) {}
}
/** @param array{nest: array{sink: NullsafeDeviceSink|null}, other: 0} $grid */
function nullsafeNestedDevice(array $grid): void {
    $inspect = new NullsafeDeviceInspect();
    $grid['nest']['sink']?->foo($grid['other'] = 1, [$grid], [$grid['nest']]); // observe
    $inspect->inspect($grid, $grid['nest'], $grid['nest']['sink']); // observe
}
function nullsafePropertyDevice(NullsafeDeviceBox $box): void {
    $inspect = new NullsafeDeviceInspect();
    $box->holders['sink']?->foo([$box->holders], $box->holders['sink'] = null, [$box->holders]); // observe
    $inspect->inspect($box->holders); // observe
}
function nullsafeAssignedSpines(?NullsafeDeviceSink $maybe): void {
    $inspect = new NullsafeDeviceInspect();
    ($sink = $maybe)?->foo($sink, [$sink]); // observe
    ($holder = new NullsafeDeviceSink())->child?->foo($holder->child, [$holder->child]); // observe
    ($holders = ['sink' => $maybe])['sink']?->foo([$holders]); // observe
    $inspect->inspect($sink, $holder, $holders); // observe
}
function nullsafePossibleVariable(bool $condition): void {
    if ($condition) {
        $sink = new NullsafeDeviceSink();
    }
    $sink?->foo($sink, [$sink]); // observe
}
function nullsafeSelectorDevice(?NullsafeDeviceSink $sink, bool $condition): void {
    $names = ['method' => $condition ? 'foo' : 'bar'];
    $sink?->{$names['method']}([$sink], ...['later' => $sink]); // observe
    $sink?->{($condition ? 'foo' : 'bar')}([$sink]); // observe
}
