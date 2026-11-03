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
use function array_all;
use function array_map;
use function assert;
use function preg_match;
use function rtrim;
use function str_starts_with;
use function strlen;
use function substr;
use RecursiveDirectoryIterator;
use RecursiveFilterIterator;
use SplFileInfo;

/**
 * @extends RecursiveFilterIterator<string, SplFileInfo, RecursiveDirectoryIterator>
 *
 * @internal This class is not covered by the backward compatibility promise for phpunit/php-file-iterator
 */
final class ExcludeIterator extends RecursiveFilterIterator
{
    /**
     * Matches a path component that begins with a dot, capturing the separator
     * that follows it (empty when the component is the last one).
     *
     * On Windows, both '\' and '/' separate path components and realpath()
     * returns '\'. On other operating systems, '\' is a legal character in a
     * file name and must therefore not be treated as a separator.
     */
    private const string HIDDEN_COMPONENT_PATTERN = DIRECTORY_SEPARATOR === '\\' ? '=[\\\\/]\.[^\\\\/]*([\\\\/]|$)=' : '=/\.[^/]*(/|$)=';
    private string $basePath;

    /**
     * @var list<string>
     */
    private array $exclude;

    /**
     * @param string       $basePath must be a resolved path (as returned by realpath())
     * @param list<string> $exclude
     */
    public function __construct(RecursiveDirectoryIterator $iterator, string $basePath, array $exclude)
    {}

    public function accept(): bool
    {}

    public function hasChildren(): bool
    {}

    public function getChildren(): self
    {}

    public function getInnerIterator(): RecursiveDirectoryIterator
    {}

    /**
     * Hidden directories, and everything below them, are not accepted. Hidden
     * files are accepted.
     *
     * This is checked on the path relative to the base path so that a hidden
     * directory in the base path itself does not hide everything below it.
     *
     * Rejecting a hidden directory here (and not only the files below it)
     * prunes it from the traversal instead of descending into it and then
     * discarding its contents.
     */
    private function isHidden(string $path, SplFileInfo $current): bool
    {}
}
