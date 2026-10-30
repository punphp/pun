<?php

declare(strict_types=1);

namespace DispatchRetention;

const RETAINED_VALUE = 1;

trait Retained
{
    public function contract(): int
    {
        return 'wrong';
    }
}
