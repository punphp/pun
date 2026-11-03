<?php

declare(strict_types=1);

namespace League\Flysystem;

use RuntimeException;
use Throwable;

final class UnableToListContents extends RuntimeException implements FilesystemOperationFailed
{
    public static function atLocation(string $location, bool $deep, Throwable $previous): UnableToListContents
    {}

    public function operation(): string
    {}
}
