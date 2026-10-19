<?php

function reported(): bool
{
    foreach (debug_backtrace(DEBUG_BACKTRACE_IGNORE_ARGS, limit: 20) as $frame) {
        if ($frame['function'] === 'report' && ! isset($frame['type'])) {
            return true;
        }
    }

    return false;
}

function words(string $s): bool
{
    $parts = explode(' ', $s);
    foreach ($parts as $part) {
        if ($part === '') {
            return true;
        }
    }

    return false;
}

function maybe(string $p): bool
{
    foreach (glob($p) as $file) {
        if ($file === 'x') {
            return true;
        }
    }

    return false;
}

function shadowed(): bool
{
    foreach (userland() as $file) {
        if ($file === 'x') {
            return true;
        }
    }

    return false;
}
-----
<?php

function reported(): bool
{
    return array_any(debug_backtrace(DEBUG_BACKTRACE_IGNORE_ARGS, limit: 20), fn($frame) => $frame['function'] === 'report' && ! isset($frame['type']));
}

function words(string $s): bool
{
    $parts = explode(' ', $s);
    return array_any($parts, fn($part) => $part === '');
}

function maybe(string $p): bool
{
    foreach (glob($p) as $file) {
        if ($file === 'x') {
            return true;
        }
    }

    return false;
}

function shadowed(): bool
{
    foreach (userland() as $file) {
        if ($file === 'x') {
            return true;
        }
    }

    return false;
}
