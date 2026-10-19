<?php

function stringify(mixed $value): string {
    if (is_object($value) && method_exists($value, '__toString')) {
        return (string) $value;
    }

    return '';
}
-----
4:9: Consider using `instanceof Stringable` instead of `is_object() && method_exists()`.
