<?php

function a(string $value)
{
    $lower = mb_strtolower($value);
    $parts = explode('-', $lower);
    return array_map(function ($part) { return $part; }, $parts);
}

function b(string $value)
{
    $lower = mb_strtolower($value);
    if (str_contains($lower, '-')) {
        $parts = explode('-', $lower);
        return array_map(function ($part) { return $part; }, $parts);
    }
}

function c(array $words, int $i)
{
    $lower = mb_strtolower($words[$i]);
    return array_map(function (string $part) { return $part; }, explode('-', $lower));
}

function d(string $value)
{
    return array_map(function (string $part) { return $part; }, explode('-', mb_strtolower($value)));
}
-----
<?php

function a(string $value)
{
    $lower = mb_strtolower($value);
    $parts = explode('-', $lower);
    return array_map(function (string $part) { return $part; }, $parts);
}

function b(string $value)
{
    $lower = mb_strtolower($value);
    if (str_contains($lower, '-')) {
        $parts = explode('-', $lower);
        return array_map(function ($part) { return $part; }, $parts);
    }
}

function c(array $words, int $i)
{
    $lower = mb_strtolower($words[$i]);
    return array_map(function (string $part) { return $part; }, explode('-', $lower));
}

function d(string $value)
{
    return array_map(function (string $part) { return $part; }, explode('-', mb_strtolower($value)));
}
