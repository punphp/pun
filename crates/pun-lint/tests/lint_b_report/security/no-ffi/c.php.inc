<?php

namespace App {
    $a = new FFI();
    $b = FFI\CData::class;
}

namespace {
    use FFI as Foreign;
    $c = new Foreign();
    $d = new FFI\CType();
}
-----
10:14: Potentially unsafe use of FFI class `FFI`.
11:14: Potentially unsafe use of FFI class `FFI\CType`.
