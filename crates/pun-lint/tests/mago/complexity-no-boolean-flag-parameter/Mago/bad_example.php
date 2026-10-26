<?php

function get_difference(string $a, string $b, bool $ignore_case): string {
    if ($ignore_case) {
        return strtolower($a) === strtolower($b) ? '' : $a;
    }

    return $a === $b ? '' : $a;
}
