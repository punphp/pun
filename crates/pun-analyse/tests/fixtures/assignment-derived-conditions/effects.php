<?php declare(strict_types=1);

namespace AssignmentDerivedConditions;

final class EffectBox
{
    /** @phpstan-impure */
    public function effect(): int { echo ''; return 1; }
}

function effects(bool $flag): bool
{
    $box = new EffectBox;
    if ($flag) {
        $method = 'first';
        $key = 'left';
    } else {
        $method = 'second';
        $key = 'right';
    }
    $callback = static function (string $value): string { echo ''; return $value; };
    $dynamic = [$key, $callback($key)];
    $methodCall = [$key, $box->effect()];
    $printed = [$key, print ''];
    $nested = ($local = $key);
    $present = [$key, isset($local)];
    $closure = [$key, static function (): bool { echo ''; return true; }];
    $arrow = [$key, static fn (): int => print ''];
    $reference = &$key;
    if ($method === 'first') {
        \PHPStan\dumpType($dynamic[0]);
        \PHPStan\dumpType($methodCall[0]);
        \PHPStan\dumpType($printed[0]);
        \PHPStan\dumpType($nested);
        \PHPStan\dumpType($present[0]);
        \PHPStan\dumpType($closure[0]);
        \PHPStan\dumpType($arrow[0]);
        \PHPStan\dumpType($reference);
    } else {
        \PHPStan\dumpType($dynamic[0]);
        \PHPStan\dumpType($methodCall[0]);
        \PHPStan\dumpType($printed[0]);
        \PHPStan\dumpType($nested);
        \PHPStan\dumpType($present[0]);
        \PHPStan\dumpType($closure[0]);
        \PHPStan\dumpType($arrow[0]);
        \PHPStan\dumpType($reference);
    }
    return true;
}
