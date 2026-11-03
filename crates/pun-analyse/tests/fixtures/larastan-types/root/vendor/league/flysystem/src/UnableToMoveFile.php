<?php

declare(strict_types=1);

namespace League\Flysystem;

use RuntimeException;
use Throwable;

final class UnableToMoveFile extends RuntimeException implements FilesystemOperationFailed
{
    /**
     * @var string
     */
    private $source;

    /**
     * @var string
     */
    private $destination;

    public static function sourceAndDestinationAreTheSame(string $source, string $destination): UnableToMoveFile
    {}

    public function source(): string
    {}

    public function destination(): string
    {}

    public static function fromLocationTo(
        string $sourcePath,
        string $destinationPath,
        ?Throwable $previous = null
    ): UnableToMoveFile {}

    public static function because(
        string $reason,
        string $sourcePath,
        string $destinationPath,
    ): UnableToMoveFile {}

    public function operation(): string
    {}
}
