<?php

declare(strict_types=1);

namespace GuzzleHttp\Promise;

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
