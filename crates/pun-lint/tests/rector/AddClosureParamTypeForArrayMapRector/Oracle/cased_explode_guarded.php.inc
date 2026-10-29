<?php

namespace Probe;

function guarded(string $word)
{
    if (str_contains($word, '-')) {
        $parts = explode('-', $word);

        return array_map(function ($part) {
            return strtoupper($part);
        }, $parts);
    }

    return [];
}

function direct(string $word)
{
    if (str_contains($word, '-')) {
        return array_map(function ($part) {
            return strtoupper($part);
        }, explode('-', $word));
    }

    return [];
}

function otherDelimiter(string $word)
{
    if (str_contains($word, '-')) {
        return array_map(function ($part) {
            return strtoupper($part);
        }, explode('_', $word));
    }

    return [];
}

function unguarded(string $word)
{
    return array_map(function ($part) {
        return strtoupper($part);
    }, explode('-', $word));
}

function lowered(string $value)
{
    $word = mb_strtolower($value);
    if (str_contains($word, '-')) {
        return array_map(function ($part) {
            return strtoupper($part);
        }, explode('-', $word));
    }

    return [];
}
-----
<?php

namespace Probe;

function guarded(string $word)
{
    if (str_contains($word, '-')) {
        $parts = explode('-', $word);

        return array_map(function ($part) {
            return strtoupper($part);
        }, $parts);
    }

    return [];
}

function direct(string $word)
{
    if (str_contains($word, '-')) {
        return array_map(function ($part) {
            return strtoupper($part);
        }, explode('-', $word));
    }

    return [];
}

function otherDelimiter(string $word)
{
    if (str_contains($word, '-')) {
        return array_map(function ($part) {
            return strtoupper($part);
        }, explode('_', $word));
    }

    return [];
}

function unguarded(string $word)
{
    return array_map(function ($part) {
        return strtoupper($part);
    }, explode('-', $word));
}

function lowered(string $value)
{
    $word = mb_strtolower($value);
    if (str_contains($word, '-')) {
        return array_map(function (string $part) {
            return strtoupper($part);
        }, explode('-', $word));
    }

    return [];
}
