<?php

namespace App;

function direct()
{
    return [];
}

function param(array $items, $flag)
{
    if ($flag) {
        return $items;
    }

    return [1];
}

/**
 * @param array $items
 */
function docOnly($items, $flag)
{
    if ($flag) {
        return $items;
    }

    return [1];
}

/**
 * @return array<int>
 */
function helper(): array
{
    return [];
}

function viaCall($flag)
{
    if ($flag) {
        return helper();
    }

    return array();
}

function builtin($flag, string $s)
{
    if ($flag) {
        return explode(',', $s);
    }

    return array_map(fn ($x) => $x, [1]);
}

function variable($flag)
{
    $items = [];
    $items[] = 1;
    if ($flag) {
        return $items;
    }

    return ['a' => 1];
}

function nullable(?array $items)
{
    if ($items === null) {
        return [];
    }

    return $items;
}

function nested($flag)
{
    if ($flag) {
        return [];
    }
}

function notTop($flag)
{
    if ($flag) {
        return [];
    }

    return [2];
}

function mixedReturn($flag)
{
    if ($flag) {
        return null;
    }

    return [];
}

final class Repo
{
    /** @var array */
    private $cache = [];

    private array $typed = [];

    public function all()
    {
        return [];
    }

    public function cached($flag)
    {
        if ($flag) {
            return $this->cache;
        }

        return [];
    }

    public function typed($flag)
    {
        if ($flag) {
            return $this->typed;
        }

        return [];
    }
}
-----
<?php

namespace App;

function direct(): array
{
    return [];
}

function param(array $items, $flag): array
{
    if ($flag) {
        return $items;
    }

    return [1];
}

/**
 * @param array $items
 */
function docOnly($items, $flag)
{
    if ($flag) {
        return $items;
    }

    return [1];
}

/**
 * @return array<int>
 */
function helper(): array
{
    return [];
}

function viaCall($flag): array
{
    if ($flag) {
        return helper();
    }

    return array();
}

function builtin($flag, string $s)
{
    if ($flag) {
        return explode(',', $s);
    }

    return array_map(fn ($x) => $x, [1]);
}

function variable($flag): array
{
    $items = [];
    $items[] = 1;
    if ($flag) {
        return $items;
    }

    return ['a' => 1];
}

function nullable(?array $items)
{
    if ($items === null) {
        return [];
    }

    return $items;
}

function nested($flag)
{
    if ($flag) {
        return [];
    }
}

function notTop($flag): array
{
    if ($flag) {
        return [];
    }

    return [2];
}

function mixedReturn($flag)
{
    if ($flag) {
        return null;
    }

    return [];
}

final class Repo
{
    /** @var array */
    private $cache = [];

    private array $typed = [];

    public function all(): array
    {
        return [];
    }

    public function cached($flag)
    {
        if ($flag) {
            return $this->cache;
        }

        return [];
    }

    public function typed($flag): array
    {
        if ($flag) {
            return $this->typed;
        }

        return [];
    }
}
