<?php

namespace Level1Parity;

/** @param list<int> $l */
function loopIsset(array $l): void
{
    foreach ($l as $v) {
        if (isset($y)) {
            echo $y;
        }
        $y = $v;
    }
}

/** @param list<int> $l */
function loopGuardedRead(array $l): void
{
    foreach ($l as $v) {
        if (isset($last) && $v < $last) {
            return;
        }
        $last = $v;
    }
}

function elseIfIsset(bool $b, mixed $delay): void
{
    $p = [];
    if (isset($delay)) {
        echo 1;
    } elseif ($b) {
        echo 2;
    } else {
        $p[] = 1;
    }
    if (! empty($p)) {
        echo 3;
    }
}

function coalesceFalse(bool $b): string
{
    if ($b) {
        $d = 'x';
    }
    if (($d ?? false) === false) {
        throw new \Exception();
    }

    return $d;
}

function coalesceNotFalse(bool $b): string
{
    if ($b) {
        $d = 'x';
    }
    if (($d ?? false) !== false) {
        return $d;
    }

    return '';
}

function tag(string $v, string $cipher): string
{
    openssl_encrypt($v, $cipher, 'key', 0, 'iv', $tag);
    openssl_encrypt($v, 'aes-256-gcm', 'key', 0, 'iv', $gcm);
    openssl_encrypt($v, 'aes-256-cbc', 'key', 0, 'iv', $cbc);

    return ($tag ?? '') . ($gcm ?? '') . ($cbc ?? '');
}

/** @param array<string, mixed> $config */
function replace(array $config): mixed
{
    $config['direct'] = array_replace(['host' => 'a'], $config['direct'] ?? []);

    return $config['direct']['options'] ?? null;
}

/**
 * @param list<int> $list
 * @param non-empty-array<string, int> $map
 * @param list<array<string, int>> $many
 */
function replaceVariants(array $list, array $map, array $many, int|array $either): void
{
    $a = array_replace(['a' => 1], ['b' => 2]);
    $b = array_replace($list, ['x' => 1]);
    $c = array_replace(['k' => 1], $map);
    $d = array_replace(...$many);
    $e = array_replace(['k' => 1], $either);
    echo $a['a'] ?? 0, $b['x'] ?? 0, $c['k'] ?? 0, $d['z'] ?? 0, $e['k'] ?? 0;
}

interface NotOffsetAccessible {}

function offsetOnObject(NotOffsetAccessible $o): void
{
    $c = $o['x'];
    if (empty($c)) {
        echo 1;
    }
}

/**
 * @template TClass of object
 * @param string|class-string<TClass>|null $abstract
 * @return ($abstract is class-string<TClass> ? TClass : ($abstract is null ? \stdClass : mixed))
 */
function app($abstract = null)
{
    return null;
}

function service(): void
{
    $session = app('session');
    if (isset($session)) {
        echo 1;
    }
}

class Registry
{
    public static function reset(): void
    {
        (function () {
            $this->items = [];
        })->call(new \stdClass());
    }
}
