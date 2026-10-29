<?php

namespace Probe;

function returned(string $value)
{
    $word = mb_strtolower($value);
    if (str_contains($word, '-')) {
        $parts = explode('-', $word);
        return array_map(function ($part) {
            return $part;
        }, $parts);
    }
}

function assigned(string $value)
{
    $word = mb_strtolower($value);
    if (str_contains($word, '-')) {
        $parts = explode('-', $word);
        $other = array_map(function ($part) {
            return $part;
        }, $parts);
    }
}

function statement(string $value)
{
    $word = mb_strtolower($value);
    if (str_contains($word, '-')) {
        $parts = explode('-', $word);
        array_map(function ($part) {
            return $part;
        }, $parts);
    }
}

function returnedPlain(string $value)
{
    $parts = explode('-', mb_strtolower($value));
    return array_map(function ($part) {
        return $part;
    }, $parts);
}
-----
<?php

namespace Probe;

function returned(string $value)
{
    $word = mb_strtolower($value);
    if (str_contains($word, '-')) {
        $parts = explode('-', $word);
        return array_map(function (string $part) {
            return $part;
        }, $parts);
    }
}

function assigned(string $value)
{
    $word = mb_strtolower($value);
    if (str_contains($word, '-')) {
        $parts = explode('-', $word);
        $other = array_map(function (string $part) {
            return $part;
        }, $parts);
    }
}

function statement(string $value)
{
    $word = mb_strtolower($value);
    if (str_contains($word, '-')) {
        $parts = explode('-', $word);
        array_map(function (string $part) {
            return $part;
        }, $parts);
    }
}

function returnedPlain(string $value)
{
    $parts = explode('-', mb_strtolower($value));
    return array_map(function (string $part) {
        return $part;
    }, $parts);
}
