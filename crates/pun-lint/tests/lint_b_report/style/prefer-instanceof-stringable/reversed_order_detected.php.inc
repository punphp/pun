<?php

if (method_exists($x, '__toString') && is_object($x)) {
    echo (string) $x;
}
-----
3:5: Consider using `instanceof Stringable` instead of `is_object() && method_exists()`.
