<?php

namespace App;

use FFI;
use FFI\CData as Data;

$a = FFI::cdef('');
$b = new Data();
$c = new \FFI();
$d = new CType();
$e = new namespace\FFI();
function f(Data $x): FFI {}
-----
8:6: Potentially unsafe use of FFI class `FFI`.
9:10: Potentially unsafe use of FFI class `FFI\CData`.
10:10: Potentially unsafe use of FFI class `FFI`.
13:12: Potentially unsafe use of FFI class `FFI\CData`.
13:22: Potentially unsafe use of FFI class `FFI`.
