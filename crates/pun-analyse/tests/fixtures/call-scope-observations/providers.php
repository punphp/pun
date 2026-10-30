<?php
class PricingSink {
    public function foo($x) {}
    public function bar($x) {}
}
function pricingFunction($v): mixed { return null; }
function pricingHook($v): mixed { return null; }
function pricingExpression($v): mixed { return null; }
class PricingWorker {
    public function fromProvider($v): mixed { return null; }
    public function fromHook($v): mixed { return null; }
    public static function staticProvider($v): mixed { return null; }
    public static function staticHook($v): mixed { return null; }
}
class PricingValue {
    public function __construct($v) {}
}
function pricingControls(PricingSink $o, PricingWorker $w, bool $b): void {
    $m = $b ? 'foo' : 'bar';
    $o->{($v = $m)}([pricingFunction($v)]); // observe
    $o->{($v = $m)}([pricingHook($v)]); // observe
    $o->{($v = $m)}([pricingExpression($v)]); // observe
    $o->{($v = $m)}([$w->fromProvider($v)]); // observe
    $o->{($v = $m)}([$w->fromHook($v)]); // observe
    $o->{($v = $m)}([PricingWorker::staticProvider($v)]); // observe
    $o->{($v = $m)}([PricingWorker::staticHook($v)]); // observe
    $o->{($v = $m)}([$w?->fromProvider($v)]); // observe
    $o->{($v = $m)}([$w?->fromHook($v)]); // observe
    $o->{($v = $m)}([new PricingValue($v)]); // observe
}
