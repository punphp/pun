<?php
$reference = function (&$value): void { $value = 19; };
$read = function ($value): void { echo $value; };
$value = 17;
$reference($value); // observe
$value = 23;
$read($value); // observe
$factory = fn () => $read;
$factory()($value); // observe
$holder = new stdClass();
$holder->callback = $read;
($holder->callback)($value); // observe
$string = 'strlen';
$text = 'retained';
$string($text); // observe
$scan = sscanf(...);
$format = '%d';
$scan('29', $format, $output); // observe
\PHPStan\dumpType($value);
\PHPStan\dumpType($text);
