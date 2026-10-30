<?php declare(strict_types=1);

namespace ClosureCaptureUnreachable;

function terminated(int $value): void
{
    return;
    $unreachable = function () use ($value): void {};
}
