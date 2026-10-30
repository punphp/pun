<?php

namespace QuotedShapeKeys;

/** @param array{"\xff": 'ff', "\xfe": 'fe', "\u{FFFD}": 'replacement'} $shape */
function binary(array $shape): void
{
    $distinct = [...[...$shape], "\xff" => 'override'];
    $otherByte = [...[...$shape], "\xfe" => 'override'];
    $replacement = [...[...$shape], "�" => 'override'];
}

/** @param array{'\xff': 'single', "\xff": 'double', '\n': 'slash', "\n": 'newline'} $shape */
function quotes(array $shape): void
{
    $single = [...[...$shape], '\xff' => 'override'];
    $double = [...[...$shape], "\xff" => 'override'];
}

/** @param array{"\x61": 'first', a: 'second', "\141": 'last'} $shape */
function equal(array $shape): void
{
    $overwritten = [...[...$shape], 'a' => 'override'];
}

/** @param array{"\x31": 'integer', '01': 'leading', 0: 'zero', name: 'bare', '\x31': 'single'} $shape */
function numeric(array $shape): void
{
    $integer = [...$shape, 'implicit', 2 => 'duplicate'];
    $leading = [...[...$shape], '01' => 'override'];
    $bare = [...[...$shape], 'name' => 'override'];
}

/** @param key-of<array{"\xff": int}> $key */
function scannerKeyOf(string $key): void
{
    $duplicate = [$key => 0, "\xff" => 1];
    $distinct = [$key => 0, "\xfe" => 1, "�" => 2];
}

function inlineKeyOf(string $key): void
{
    /** @var key-of<array{"\xfe": int}> $key */
    $duplicate = [$key => 0, "\xfe" => 1];
    $distinct = [$key => 0, "\xff" => 1, "�" => 2];
}

function inlineShape(array $shape): void
{
    /** @var array{"\x31": int, '01': int, label: int} $shape */
    $numeric = [...$shape, 'implicit', 1 => 'duplicate'];
}

/** @param array{"\x61": 'escaped', label: 'bare'} $shape */
function ascii(array $shape): void
{
    $equal = [...[...$shape], 'a' => 'override'];
    $different = [...[...$shape], '\x61' => 'distinct'];
}
