<?php

namespace Probe;

function loopReassign(array $words, int $n)
{
    for ($i = 0; $i < $n; $i++) {
        $word = mb_strtolower($words[$i]);
        if (str_contains($word, '-')) {
            $parts = explode('-', $word);
            $parts = array_map(function ($part) {
                return $part;
            }, $parts);
        }
    }
}

function loopOnly(array $words, int $n)
{
    for ($i = 0; $i < $n; $i++) {
        $word = mb_strtolower($words[$i]);
        if (str_contains($word, '-')) {
            $parts = explode('-', $word);
            $other = array_map(function ($part) {
                return $part;
            }, $parts);
        }
    }
}

function reassignOnly(string $value)
{
    $word = mb_strtolower($value);
    if (str_contains($word, '-')) {
        $parts = explode('-', $word);
        $parts = array_map(function ($part) {
            return $part;
        }, $parts);
    }
}

function plainReassign(string $value)
{
    $parts = explode('-', $value);
    $parts = array_map(function ($part) {
        return $part;
    }, $parts);
}

function lowerVar(string $value)
{
    $parts = explode('-', strtolower($value));
    $other = array_map(function ($part) {
        return $part;
    }, $parts);
}

function lowerMixed($value)
{
    return array_map(function ($part) {
        return $part;
    }, explode('-', strtolower($value)));
}
-----
<?php

namespace Probe;

function loopReassign(array $words, int $n)
{
    for ($i = 0; $i < $n; $i++) {
        $word = mb_strtolower($words[$i]);
        if (str_contains($word, '-')) {
            $parts = explode('-', $word);
            $parts = array_map(function (string $part) {
                return $part;
            }, $parts);
        }
    }
}

function loopOnly(array $words, int $n)
{
    for ($i = 0; $i < $n; $i++) {
        $word = mb_strtolower($words[$i]);
        if (str_contains($word, '-')) {
            $parts = explode('-', $word);
            $other = array_map(function (string $part) {
                return $part;
            }, $parts);
        }
    }
}

function reassignOnly(string $value)
{
    $word = mb_strtolower($value);
    if (str_contains($word, '-')) {
        $parts = explode('-', $word);
        $parts = array_map(function (string $part) {
            return $part;
        }, $parts);
    }
}

function plainReassign(string $value)
{
    $parts = explode('-', $value);
    $parts = array_map(function ($part) {
        return $part;
    }, $parts);
}

function lowerVar(string $value)
{
    $parts = explode('-', strtolower($value));
    $other = array_map(function (string $part) {
        return $part;
    }, $parts);
}

function lowerMixed($value)
{
    return array_map(function (string $part) {
        return $part;
    }, explode('-', strtolower($value)));
}
