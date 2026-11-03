<?php

declare(strict_types=1);

namespace League\Flysystem;

use RuntimeException;
use Throwable;

final class UnableToGeneratePublicUrl extends RuntimeException implements FilesystemException
{
    public function __construct(string $reason, string $path, ?Throwable $previous = null)
    {}

    public static function dueToError(string $path, Throwable $exception): static
    {}

    public static function noGeneratorConfigured(string $path, string $extraReason = ''): static
    {}
}
