<?php

namespace PunFunctionChecks;

use function sprintf as render;

function formats(string $dynamic, array $values): void
{
    render('%*.*f', 4, 2);
    \printf('%0*d', 8);
    \printf('%0*d', 8, 2);
    \printf("%'x*d", 8, 2);
    \printf('%h %H', 1, 2);
    \printf('%q', 1);
    \printf('%%q', 1);
    \printf('%0$s', 1);
    \printf('%2$s %s', 1);
    \printf($dynamic, 1);
    \printf();
    \printf('%s %s', ...$values);
    \printf(\stdClass::class, 1);
    \printf(format: '%s');
    \vsprintf();
    \vsprintf($dynamic, []);
    \vsprintf('plain');
    \vsprintf('%s');
    \vsprintf('%s', $values);
    \vsprintf('plain', $values);
    \vsprintf('%s', ['named' => 1, 'other' => 2]);
    \vsprintf('%s', 'not an array');
    \vsprintf('%q', []);
    \sscanf('', '%[]]', $a, $b);
    \sscanf('', '%[^]]', $a, $b);
    \sscanf('', '%[]', $a);
    \sscanf('', '%[^]', $a);
    \sscanf('', '%[a-z', $a);
    \sscanf('', '%[', $a);
    \sscanf('', '%[^', $a);
    \sscanf('', '%d %*s %n', $a);
    \sscanf('', '%*n %3$d %1$s', $a);
    \sscanf('', '%1$d %1$s', $a);
    \sscanf('', '%d %2$s', $a);
    \sscanf('', '%2$d %s', $a);
    \sscanf('', '%0$d', $a);
    \sscanf('', '%256$d', $a);
    \sscanf('', '%4294967297$d', $a, $b);
    \sscanf('', '%18446744073709551616$d', $a);
    \sscanf('', '%0c %10n %Ld', $a);
    \sscanf('', '%lld', $a);
    \sscanf('', '%F', $a);
    \sscanf('', '%', $a);
    \sscanf('', '%*', $a);
    \sscanf('', '%12', $a);
    \sscanf('', '%l', $a);
    \sscanf('', '%1$', $a);
    \sscanf('', '%1$*d', $a);
    \sscanf('', '%% %*d', $a);
    \sscanf('', "%d\0%q", $a, $b);
    \sscanf('', '%*12d %5s', $a, $b);
    \sscanf('', '%3$12d', $a);
    $callee = 'printf';
    $callee('%s', 1);
}

/** @param '%s'|'%s%s%s' $format */
function ranges(string $format, bool $choice): void
{
    $values = $choice ? [1] : [1, 2];
    \vsprintf($format, $values);
    \vsprintf($format, [1]);
    \vsprintf($format, []);
    \vsprintf('%s%s%s%s', $values);
    \vsprintf('plain', $values);
    \vsprintf('%s', $choice ? [1] : [1, 2, 3]);
    \printf($format, 1);
}

/** @param array{a: int, b?: int} $values */
function optional_items(array $values): void
{
    \vsprintf('%s', $values);
}

/** @param list<int> $values */
function general_list(array $values): void
{
    \vsprintf('%s', $values);
    \printf(123, 1);
    missing();
    \PunMissingNamespace\missing();
}

function unreachable(): void
{
    return;
    \printf('%s');
    \vsprintf('%s', []);
}

namespace PunShadowFunction;

function sprintf(...$args): string { return ''; }
function vsprintf(...$args): string { return ''; }

sprintf('%s');
vsprintf('%s');
