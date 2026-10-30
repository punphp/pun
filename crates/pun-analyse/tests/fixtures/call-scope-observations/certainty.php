<?php
class CertaintySink {
    public function foo($value, $later = null) {}
    public function bar($value, $later = null) {}
}
function conditionalVariable(CertaintySink $sink, bool $condition): void {
    if ($condition) {
        $name = 'foo';
        $value = 1;
    } else {
        $name = 'bar';
    }
    $sink->$name([$value]); // observe
    $sink->foo(($value ??= 3), [$value]); // observe
}
function removedVariable(CertaintySink $sink, bool $condition): void {
    $value = 1;
    if ($condition) {
        $name = 'foo';
    } else {
        $name = 'bar';
        unset($value);
    }
    $sink->$name([$value]); // observe
}
