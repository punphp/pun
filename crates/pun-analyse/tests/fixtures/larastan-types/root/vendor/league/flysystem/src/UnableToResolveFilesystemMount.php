<?php

declare(strict_types=1);

namespace League\Flysystem;

use RuntimeException;

class UnableToResolveFilesystemMount extends RuntimeException implements FilesystemException
{
    public static function becauseTheSeparatorIsMissing(string $path): UnableToResolveFilesystemMount
    {}

    public static function becauseTheMountWasNotRegistered(string $mountIdentifier): UnableToResolveFilesystemMount
    {}
}
