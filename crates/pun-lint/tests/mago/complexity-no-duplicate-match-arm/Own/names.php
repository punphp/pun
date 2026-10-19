<?php
namespace A;
use B\C;
$r = match ($v) {
 1 => FOO,
 2 => \FOO,
 3 => \A\FOO,
 4 => C::X,
 5 => \B\C::X,
 6 => c::X,
 7 => 0.0,
 8 => -0.0,
 9 => "1",
 10 => 1,
 11 => (int) "1",
};
