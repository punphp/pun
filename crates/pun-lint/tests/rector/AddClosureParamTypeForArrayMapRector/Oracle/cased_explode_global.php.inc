<?php

namespace P;
function b(string $value)
{
    $lower = mb_strtolower($value);
    if (str_contains($lower, '-')) {
        $parts = explode('-', $lower);
        return array_map(function ($part) { return $part; }, $parts);
    }
}
-----
<?php

namespace P;
function b(string $value)
{
    $lower = mb_strtolower($value);
    if (str_contains($lower, '-')) {
        $parts = explode('-', $lower);
        return array_map(function (string $part) { return $part; }, $parts);
    }
}
