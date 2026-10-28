<?php
namespace Pun\Ns3;
class Probe {
    public static function mixedParam(mixed $m) {}
    final public static function finalStatic($m) {}
    /** @phpstan-assert true $m */
    public static function asserted($m) {}
    /** @phpstan-assert true $m */
    public static function assertedMixed(mixed $m): void {}
    public static function voidRet($m): void {}
}
