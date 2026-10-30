<?php

namespace Probe;

function headline(string $value)
{
    $words = preg_split('/\s+/u', $value, -1, PREG_SPLIT_NO_EMPTY);
    $wordCount = count($words);

    for ($i = 0; $i < $wordCount; $i++) {
        $lowercaseWord = mb_strtolower($words[$i]);

        if (str_contains($lowercaseWord, '-')) {
            $hyphenatedWords = explode('-', $lowercaseWord);

            $hyphenatedWords = array_map(function ($part) {
                return mb_strtoupper($part);
            }, $hyphenatedWords);

            $words[$i] = implode('-', $hyphenatedWords);
        }
    }

    return $words;
}

function fromMixed($value)
{
    return array_map(function ($part) {
        return $part;
    }, explode('-', strtolower($value)));
}

function upper(string $value)
{
    return array_map(function ($part) {
        return $part;
    }, explode('-', strtoupper($value)));
}
-----
<?php

namespace Probe;

function headline(string $value)
{
    $words = preg_split('/\s+/u', $value, -1, PREG_SPLIT_NO_EMPTY);
    $wordCount = count($words);

    for ($i = 0; $i < $wordCount; $i++) {
        $lowercaseWord = mb_strtolower($words[$i]);

        if (str_contains($lowercaseWord, '-')) {
            $hyphenatedWords = explode('-', $lowercaseWord);

            $hyphenatedWords = array_map(function (string $part) {
                return mb_strtoupper($part);
            }, $hyphenatedWords);

            $words[$i] = implode('-', $hyphenatedWords);
        }
    }

    return $words;
}

function fromMixed($value)
{
    return array_map(function (string $part) {
        return $part;
    }, explode('-', strtolower($value)));
}

function upper(string $value)
{
    return array_map(function (string $part) {
        return $part;
    }, explode('-', strtoupper($value)));
}
