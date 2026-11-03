<?php

declare(strict_types=1);

namespace League\Flysystem;

use InvalidArgumentException;

use function var_export;

class InvalidVisibilityProvided extends InvalidArgumentException implements FilesystemException
{
    public static function withVisibility(string $visibility, string $expectedMessage): InvalidVisibilityProvided
    {}
}
