<?php

declare(strict_types=1);

namespace League\Flysystem;

use function rtrim;
use function strlen;
use function substr;

final class PathPrefixer
{
    private string $prefix = '';

    public function __construct(string $prefix, private string $separator = '/')
    {}

    public function prefixPath(string $path): string
    {}

    public function stripPrefix(string $path): string
    {}

    public function stripDirectoryPrefix(string $path): string
    {}

    public function prefixDirectoryPath(string $path): string
    {}
}
