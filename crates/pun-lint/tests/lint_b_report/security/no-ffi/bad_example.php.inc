<?php

use FFI;

$ffi = FFI::cdef("void* malloc(size_t size);");
$ffi->malloc(1024); // Allocate memory but never free it
-----
5:8: Potentially unsafe use of FFI class `FFI`.
