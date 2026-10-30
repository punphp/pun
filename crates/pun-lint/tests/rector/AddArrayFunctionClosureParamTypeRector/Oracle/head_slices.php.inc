<?php

namespace Probe;

/**
 * @param non-empty-list<string> $nonEmpty
 * @param list<string> $list
 */
function slices(string $v, array $nonEmpty, array $list)
{
    array_filter(array_slice(explode("\n", $v), 0, 2), fn ($a) => $a !== '');
    array_filter(array_slice(explode("\n", $v), 1), fn ($b) => $b !== '');
    array_filter(array_slice(explode("\n", $v), 0), fn ($c) => $c !== '');
    array_filter(array_slice(explode("\n", $v), 1, 2), fn ($d) => $d !== '');
    array_filter(array_slice($nonEmpty, 0, 2), fn ($e) => $e !== '');
    array_filter(array_slice($list, 0, 2), fn ($f) => $f !== '');
    array_filter(array_slice(explode("\n", $v), 0, -1), fn ($g) => $g !== '');
}

function guarded(array $items)
{
    $names = [];
    foreach ($items as $item) {
        $names[] = (string) $item;
    }

    if (empty($names)) {
        return;
    }

    array_filter($names, fn ($n) => $n !== '');
}

function notGuarded(array $items)
{
    $names = [];
    foreach ($items as $item) {
        $names[] = (string) $item;
    }

    array_filter($names, fn ($n) => $n !== '');
}

function negated(array $items)
{
    $names = [];
    foreach ($items as $item) {
        $names[] = (string) $item;
    }

    if (! $names) {
        return;
    }

    array_filter($names, fn ($n) => $n !== '');
}
-----
<?php

namespace Probe;

/**
 * @param non-empty-list<string> $nonEmpty
 * @param list<string> $list
 */
function slices(string $v, array $nonEmpty, array $list)
{
    array_filter(array_slice(explode("\n", $v), 0, 2), fn ($a) => $a !== '');
    array_filter(array_slice(explode("\n", $v), 1), fn (string $b) => $b !== '');
    array_filter(array_slice(explode("\n", $v), 0), fn ($c) => $c !== '');
    array_filter(array_slice(explode("\n", $v), 1, 2), fn (string $d) => $d !== '');
    array_filter(array_slice($nonEmpty, 0, 2), fn ($e) => $e !== '');
    array_filter(array_slice($list, 0, 2), fn (string $f) => $f !== '');
    array_filter(array_slice(explode("\n", $v), 0, -1), fn (string $g) => $g !== '');
}

function guarded(array $items)
{
    $names = [];
    foreach ($items as $item) {
        $names[] = (string) $item;
    }

    if (empty($names)) {
        return;
    }

    array_filter($names, fn ($n) => $n !== '');
}

function notGuarded(array $items)
{
    $names = [];
    foreach ($items as $item) {
        $names[] = (string) $item;
    }

    array_filter($names, fn (string $n) => $n !== '');
}

function negated(array $items)
{
    $names = [];
    foreach ($items as $item) {
        $names[] = (string) $item;
    }

    if (! $names) {
        return;
    }

    array_filter($names, fn ($n) => $n !== '');
}
