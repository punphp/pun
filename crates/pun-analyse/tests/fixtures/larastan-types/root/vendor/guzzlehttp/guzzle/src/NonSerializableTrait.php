<?php

declare(strict_types=1);

namespace GuzzleHttp;

/**
 * @internal
 */
trait NonSerializableTrait
{
    public function __serialize(): array
    {}

    public function __unserialize(array $data): void
    {}
}
