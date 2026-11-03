<?php

declare(strict_types=1);

namespace League\Flysystem;

use RuntimeException;
use Throwable;

final class UnableToCreateDirectory extends RuntimeException implements FilesystemOperationFailed
{
    private string $location;
    private string $reason = '';

    public static function atLocation(string $dirname, string $errorMessage = '', ?Throwable $previous = null): UnableToCreateDirectory
    {}

    public static function dueToFailure(string $dirname, Throwable $previous): UnableToCreateDirectory
    {}

    public function operation(): string
    {}

    public function reason(): string
    {}

    public function location(): string
    {}
}
