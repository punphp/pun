<?php declare(strict_types=1);
/*
 * This file is part of sebastian/file-filter.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace SebastianBergmann\FileFilter;

use const DIRECTORY_SEPARATOR;
use function preg_quote;
use function str_replace;
use function strlen;
use function strrpos;
use function substr;

/**
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for this library
 */
final readonly class Builder
{
    /**
     * @param list<array{path: non-empty-string, prefix: string, suffix: string}> $includeDirectories
     * @param list<non-empty-string>                                              $includeFiles
     * @param list<array{path: non-empty-string, prefix: string, suffix: string}> $excludeDirectories
     * @param list<non-empty-string>                                              $excludeFiles
     */
    public function build(array $includeDirectories, array $includeFiles, array $excludeDirectories, array $excludeFiles): Filter
    {}

    /**
     * @param list<array{path: non-empty-string, prefix: string, suffix: string}> $directories
     *
     * @return list<array{regularExpression: string, prefix: string, suffix: string}>
     */
    private function matchers(array $directories): array
    {}

    /**
     * @param list<non-empty-string> $files
     *
     * @return array<string, true>
     */
    private function map(array $files): array
    {}

    private function globToRegularExpression(string $path): string
    {}
}
