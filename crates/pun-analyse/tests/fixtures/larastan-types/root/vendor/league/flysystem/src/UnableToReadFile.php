<?php

declare(strict_types=1);

namespace League\Flysystem;

use RuntimeException;
use Throwable;

final class UnableToReadFile extends RuntimeException implements FilesystemOperationFailed
{
    /**
     * @var string
     */
    private $location = '';

    /**
     * @var string
     */
    private $reason = '';

    public static function fromLocation(string $location, string $reason = '', ?Throwable $previous = null): UnableToReadFile
    {}

    public function operation(): string
    {}

    public function reason(): string
    {}

    public function location(): string
    {}
}
