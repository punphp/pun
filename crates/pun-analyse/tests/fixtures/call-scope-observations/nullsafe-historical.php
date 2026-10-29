<?php
class nullsafehistoryfirst {
    public function seed($value) {}
    public function foo($value) {}
    public function bar($value) {}
}
class nullsafehistorysecond extends nullsafehistoryfirst {}
function nullsafeHistoryPositions(bool $condition): void {
    if ($condition) {
        $object = new nullsafehistoryfirst();
        $name = 'foo';
        $value = 1;
    } else {
        $object = new nullsafehistorysecond();
        $name = 'bar';
        $value = 2;
    }
    $object->seed([$value]); // historical
    $object = new nullsafehistorysecond();
    $object?->$name(0); // observe
    $value = 'later';
    $object?->foo(0); // observe
}
