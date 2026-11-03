<?php

declare(strict_types=1);

namespace League\Flysystem;

use RuntimeException;
use Throwable;

final class UnableToRetrieveMetadata extends RuntimeException implements FilesystemOperationFailed
{
    /**
     * @var string
     */
    private $location;

    /**
     * @var string
     */
    private $metadataType;

    /**
     * @var string
     */
    private $reason;

    public static function lastModified(string $location, string $reason = '', ?Throwable $previous = null): self
    {}

    public static function visibility(string $location, string $reason = '', ?Throwable $previous = null): self
    {}

    public static function fileSize(string $location, string $reason = '', ?Throwable $previous = null): self
    {}

    public static function mimeType(string $location, string $reason = '', ?Throwable $previous = null): self
    {}

    public static function create(string $location, string $type, string $reason = '', ?Throwable $previous = null): self
    {}

    public function reason(): string
    {}

    public function location(): string
    {}

    public function metadataType(): string
    {}

    public function operation(): string
    {}
}
