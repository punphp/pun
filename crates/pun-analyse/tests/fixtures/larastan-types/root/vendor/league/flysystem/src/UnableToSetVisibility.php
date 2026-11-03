<?php

declare(strict_types=1);

namespace League\Flysystem;

use RuntimeException;

use Throwable;

use function rtrim;

final class UnableToSetVisibility extends RuntimeException implements FilesystemOperationFailed
{
    /**
     * @var string
     */
    private $location;

    /**
     * @var string
     */
    private $reason;

    public function reason(): string
    {}

    public static function atLocation(string $filename, string $extraMessage = '', ?Throwable $previous = null): self
    {}

    public function operation(): string
    {}

    public function location(): string
    {}
}
