<?php
/** @param object{present: int} $shape */
function untyped($shape): void {
    \PHPStan\dumpType($shape);
    property_exists($shape, 'absent');
    property_exists($shape, 'present');
}
function annotated($input): void {
    /** @var object{present: int} $shape */
    $shape = $input;
    \PHPStan\dumpType($shape);
    property_exists($shape, 'absent');
    property_exists($shape, 'present');
}
/** @return object{present: int} */
function factory(): object { return (object) ['present' => 1]; }
function returned(): void {
    $shape = factory();
    \PHPStan\dumpType($shape);
    property_exists($shape, 'absent');
    property_exists($shape, 'present');
}
