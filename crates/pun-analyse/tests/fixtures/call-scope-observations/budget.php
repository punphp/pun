<?php
class BudgetSink {
    public function foo($value) {}
    public function bar($value) {}
}
function boundedPricing(BudgetSink $sink, bool $condition): void {
    $name = $condition ? 'foo' : 'bar';
    $sink->$name([7]); // observe
}
