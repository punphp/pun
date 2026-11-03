<?php declare(strict_types=1);
/*
 * This file is part of phpunit/php-code-coverage.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace SebastianBergmann\CodeCoverage;

use function array_keys;
use function is_file;
use function realpath;
use function str_contains;
use function str_starts_with;

/**
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for phpunit/php-code-coverage
 */
final class Filter
{
    /**
     * @var array<non-empty-string,true>
     */
    private array $files = [];

    /**
     * @var array<string,bool>
     */
    private array $isFileCache = [];

    /**
     * @param list<string> $filenames
     */
    public function includeFiles(array $filenames): void
    {}

    public function includeFile(string $filename): void
    {}

    public function isFile(string $filename): bool
    {}

    public function isExcluded(string $filename): bool
    {}

    /**
     * @return list<non-empty-string>
     */
    public function files(): array
    {}

    public function isEmpty(): bool
    {}
}
