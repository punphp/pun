<?php

$a = FFI::cdef('int f();');
$b = new FFI();
$c = \FFI::SOMETHING;
function f(FFI $x, ?\FFI\CData $y): FFI\CType {}
$d = new ffi\cdata();
$e = Foo::cdef();
-----
3:6: Potentially unsafe use of FFI class `FFI`.
4:10: Potentially unsafe use of FFI class `FFI`.
5:6: Potentially unsafe use of FFI class `FFI`.
6:12: Potentially unsafe use of FFI class `FFI`.
6:21: Potentially unsafe use of FFI class `FFI\CData`.
6:37: Potentially unsafe use of FFI class `FFI\CType`.
7:10: Potentially unsafe use of FFI class `ffi\cdata`.
