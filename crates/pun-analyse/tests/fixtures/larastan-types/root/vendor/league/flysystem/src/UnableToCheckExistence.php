<?php

declare(strict_types=1);

namespace League\Flysystem;

use RuntimeException;
use Throwable;

class UnableToCheckExistence extends RuntimeException implements FilesystemOperationFailed
{
    final public function __construct(string $message = "", int $code = 0, ?Throwable $previous = null)
    {}

    public static function forLocation(string $path, ?Throwable $exception = null): static
    {}

    public function operation(): string
    {}
}
