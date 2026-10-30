<?php
namespace Mago {
    function inspect(...$values): void {}
    function confirm($value, string $expected): void {}
}
namespace {
    function debugTargets(): void {
        $invoke = rand() ? 'Mago\\inspect' : 'Mago\\confirm';
        $i = 0;
        $invoke(++$i, '1');
        $i;
    }
}
