<?php
class HistoryFirst {
    public function seed($value) {}
    public function foo($value) {}
    public function bar($value) {}
}
class HistorySecond extends HistoryFirst {}
function historyPositions(bool $condition): void {
    if ($condition) {
        $object = new HistoryFirst();
        $name = 'foo';
        $value = 1;
    } else {
        $object = new HistorySecond();
        $name = 'bar';
        $value = 2;
    }
    $object->seed([$value]); // historical
    $object = new HistorySecond();
    $object->$name(0); // observe
    $value = 'later';
    $object->foo(0); // observe
}
