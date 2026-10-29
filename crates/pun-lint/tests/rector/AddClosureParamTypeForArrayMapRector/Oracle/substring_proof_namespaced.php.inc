<?php

namespace P;

function fq(string $value)
{
    $lower = mb_strtolower($value);
    if (\str_contains($lower, '-')) {
        $parts = explode('-', $lower);
        return array_map(function ($part) { return $part; }, $parts);
    }
}

function limited(string $value)
{
    return array_map(function ($part) { return $part; }, explode('-', strtolower($value), 1));
}

function upperVar(string $value)
{
    $u = strtoupper($value);
    $parts = explode(',', $u);
    return array_map(function ($part) { return $part; }, $parts);
}

function starts(string $value)
{
    $lower = strtolower($value);
    if (str_starts_with($lower, '-')) {
        return array_map(function ($part) { return $part; }, explode('-', $lower));
    }
}
-----
<?php

namespace P;

function fq(string $value)
{
    $lower = mb_strtolower($value);
    if (\str_contains($lower, '-')) {
        $parts = explode('-', $lower);
        return array_map(function ($part) { return $part; }, $parts);
    }
}

function limited(string $value)
{
    return array_map(function ($part) { return $part; }, explode('-', strtolower($value), 1));
}

function upperVar(string $value)
{
    $u = strtoupper($value);
    $parts = explode(',', $u);
    return array_map(function (string $part) { return $part; }, $parts);
}

function starts(string $value)
{
    $lower = strtolower($value);
    if (str_starts_with($lower, '-')) {
        return array_map(function (string $part) { return $part; }, explode('-', $lower));
    }
}
