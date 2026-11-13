<?php

namespace BuiltinReturns;

use function PHPStan\dumpType;

/**
 * @param int<0, 10> $range
 * @param non-empty-list<int<3, 9>> $ranges
 * @param list<array{active_users: int, signups: int, day: string}> $days
 * @param 'active_users'|'signups' $metric
 * @param 'day'|'signups' $mixed
 */
function f(int $a, int $b, float $f, int $range, array $ranges, string $s, array $days, string $metric, string $mixed): void
{
    dumpType(max(1, $a));
    dumpType(max(1, $a - $b + 1));
    dumpType(max($a, 1));
    dumpType(min(5, $a));
    dumpType(min($a, $b));
    dumpType(max(1, $f));
    dumpType(max($range, 5));
    dumpType(min($range, 5));
    dumpType(max(0, $range, $a));
    dumpType(max(1, 2, 3));
    dumpType(min(3, 1, 2));
    dumpType(max($ranges));
    dumpType(min($ranges));
    dumpType(max(1, $s));
    dumpType(time());
    dumpType(sodium_crypto_sign_keypair());
    dumpType(sodium_crypto_sign_publickey('x'));
    dumpType(strpos($s, 'a'));
    dumpType(ord($s));
    dumpType(array_column($days, $metric));
    dumpType(array_sum(array_column($days, $metric)));
    dumpType(array_column($days, $mixed));
    dumpType(array_column($days, 'signups', $metric));
}
