<?php declare(strict_types=1);

namespace ParamsBranchContracts;

final class Receiver
{
    /** @no-named-arguments */
    public function forbidden(int ...$values): void {}
    public function permitted(int ...$values): void {}
}

/**
 * @param array<string, int> $strings
 * @param list<int> $integers
 */
function selected(bool $flag, Receiver $receiver, array $strings, array $integers, array $unknown): void
{
    $method = $flag ? 'forbidden' : 'permitted';
    $receiver->$method(...$strings);
    $receiver->$method(...$integers);
    $receiver->$method(...$unknown);
    $receiver->forbidden(...$strings);
    $receiver->forbidden(...$integers);
    $receiver->forbidden(...$unknown);
}
