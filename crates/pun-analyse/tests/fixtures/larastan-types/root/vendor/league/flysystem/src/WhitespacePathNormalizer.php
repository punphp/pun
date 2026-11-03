<?php

declare(strict_types=1);

namespace League\Flysystem;

use function array_pop;
use function explode;
use function implode;
use function preg_match;
use function str_replace;

class WhitespacePathNormalizer implements PathNormalizer
{
    private bool $allowRelativePaths;

    public function __construct(bool $allowRelativePathTraversal = true)
    {}

    public function normalizePath(string $path): string
    {}
}
