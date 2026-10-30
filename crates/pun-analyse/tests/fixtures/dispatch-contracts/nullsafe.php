<?php

declare(strict_types=1);

namespace DispatchNullsafe;

class Target
{
    public function valid(): void
    {
    }
}

function probe(?Target $target, int $name): void
{
    $target?->valid();
    $target?->{'valid'}();
    $target?->{$name}();
}
