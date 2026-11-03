<?php

declare(strict_types=1);

namespace GuzzleHttp\Psr7;

/**
 * @internal
 */
trait NonSerializableStreamTrait
{
    public function __serialize(): array
    {}

    public function __unserialize(array $data): void
    {}
}
