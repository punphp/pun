<?php

declare(strict_types=1);

namespace DispatchRetention;

class Host
{
    use Retained;

    public function value(): int
    {
        return RETAINED_VALUE;
    }
}
