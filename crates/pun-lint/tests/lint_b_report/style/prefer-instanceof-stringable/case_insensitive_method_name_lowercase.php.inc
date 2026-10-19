<?php

if (is_object($x) && method_exists($x, '__tostring')) {
    echo (string) $x;
}
-----
3:5: Consider using `instanceof Stringable` instead of `is_object() && method_exists()`.
