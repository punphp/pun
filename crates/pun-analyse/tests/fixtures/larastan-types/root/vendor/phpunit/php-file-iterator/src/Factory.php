<?php declare(strict_types=1);
/*
 * This file is part of phpunit/php-file-iterator.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace SebastianBergmann\FileIterator;

use const DIRECTORY_SEPARATOR;
use const GLOB_ONLYDIR;
use function array_merge;
use function array_unique;
use function glob;
use function is_dir;
use function is_string;
use function ltrim;
use function realpath;
use function sort;
use function str_ends_with;
use function strpos;
use function substr;
use AppendIterator;
use FilesystemIterator;
use RecursiveDirectoryIterator;
use RecursiveIteratorIterator;
use UnexpectedValueException;

/**
 * @internal This class is not covered by the backward compatibility promise for phpunit/php-file-iterator
 */
final class Factory
{
    /**
     * @param list<non-empty-string>|non-empty-string $paths
     * @param list<non-empty-string>|string           $suffixes
     * @param list<non-empty-string>|string           $prefixes
     * @param list<non-empty-string>                  $exclude
     *
     * @phpstan-ignore missingType.generics
     */
    public function getFileIterator(array|string $paths, array|string $suffixes = '', array|string $prefixes = '', array $exclude = []): AppendIterator
    {}

    /**
     * @param list<non-empty-string>|string $value
     *
     * @return list<non-empty-string>
     */
    private function toList(array|string $value): array
    {}

    /**
     * A directory that cannot be opened is skipped instead of aborting the
     * traversal it is part of.
     *
     * This happens when the directory cannot be read by the current user, and
     * it happens when the directory is removed by another process after it was
     * read from its parent directory and before it is opened here.
     *
     * RecursiveIteratorIterator::CATCH_GET_CHILD does this for the directories
     * that are descended into; this method does it for the root of a traversal,
     * which is opened here and not by RecursiveIteratorIterator.
     */
    private function directoryIterator(string $path): ?RecursiveDirectoryIterator
    {}

    /**
     * @param list<non-empty-string> $paths
     *
     * @return list<non-empty-string>
     */
    private function resolveWildcards(array $paths): array
    {}

    /**
     * @see https://gist.github.com/funkjedi/3feee27d873ae2297b8e2370a7082aad
     *
     * @return list<string>
     */
    private function globstar(string $pattern): array
    {}
}
