<?php

function stringify(mixed $value): string {
    if ($value instanceof Stringable) {
        return (string) $value;
    }

    return '';
}
-----
