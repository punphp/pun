<?php
namespace A;

use function Foo\bar as iterator_to_array;
use function iterator_to_array as ita;
use function Baz\{qux as itb};

foreach (\iterator_to_array($a) as $x) {}
foreach (iterator_to_array($a) as $x) {}
foreach (ita($a) as $x) {}
foreach (itb($a) as $x) {}
foreach (B\iterator_to_array($a) as $x) {}
