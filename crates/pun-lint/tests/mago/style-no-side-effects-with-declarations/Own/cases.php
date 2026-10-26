<?php
namespace App;
use function Foo\other as class_alias;
use Bar;
declare(ticks=1);
;
{ class Inner {} }
class_alias('A', 'B');
CLASS_EXISTS('A');
\class_exists('A');
const X = 1;
function f() { echo 1; }
$f = function () { echo 1; };
$g = fn() => print(1);
$h = new class { function m() { echo 1; } };
interface I {}
trait T {}
enum E {}
while (false) { class W {} }
