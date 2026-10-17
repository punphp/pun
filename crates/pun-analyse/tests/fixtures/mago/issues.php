<?php

declare(strict_types=1);

namespace MagoIssues;

final class Box
{
    public function __construct(public int $size)
    {
    }
}

function size(Box $box): string
{
    return $box->size;
}

function missing(): void
{
    undefined_function();
    new Missing();
    echo $undefined;
    (new Box(1))->nope();
}
