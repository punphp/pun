<?php

declare(strict_types=1);

namespace League\Flysystem;

use RuntimeException;
use Throwable;

final class UnableToDeleteFile extends RuntimeException implements FilesystemOperationFailed
{
    /**
     * @var string
     */
    private $location = '';

    /**
     * @var string
     */
    private $reason;

    public static function atLocation(string $location, string $reason = '', ?Throwable $previous = null): UnableToDeleteFile
    {}

    public function operation(): string
    {}

    public function reason(): string
    {}

    public function location(): string
    {}
}
