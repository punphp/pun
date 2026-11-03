<?php

declare(strict_types=1);

namespace League\Flysystem;

use RuntimeException;

class PathTraversalDetected extends RuntimeException implements FilesystemException
{
    private string $path;

    public function path(): string
    {}

    public static function forPath(string $path): PathTraversalDetected
    {}
}
