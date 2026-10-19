<?php
namespace A;
use B\C;
use const D\BAR;
use E\{F, const G};
$r = match ($v) {
 1 => C\FOO,
 2 => \B\C\FOO,
 3 => BAR,
 4 => \D\BAR,
 5 => G,
 6 => \E\G,
 7 => F\H,
 8 => \E\F\H,
};
