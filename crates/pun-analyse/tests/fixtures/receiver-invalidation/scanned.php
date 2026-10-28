<?php

namespace ReceiverInvalidation;

class PatchedBox
{
    public ?int $value;

    public function unknown(): int { return 1; }

    /** @phpstan-impure */
    public function impure(): int { return 1; }
}
