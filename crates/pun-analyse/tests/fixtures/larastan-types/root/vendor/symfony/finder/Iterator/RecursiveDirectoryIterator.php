<?php

/*
 * This file is part of the Symfony package.
 *
 * (c) Fabien Potencier <fabien@symfony.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace Symfony\Component\Finder\Iterator;

use Symfony\Component\Finder\Exception\AccessDeniedException;
use Symfony\Component\Finder\SplFileInfo;

/**
 * Extends the \RecursiveDirectoryIterator to support relative paths.
 *
 * @author Victor Berchet <victor@suumit.com>
 *
 * @extends \RecursiveDirectoryIterator<string, SplFileInfo>
 */
class RecursiveDirectoryIterator extends \RecursiveDirectoryIterator
{
    private bool $ignoreUnreadableDirs;
    private bool $ignoreFirstRewind = true;
    private array $realPaths = [];

    // these 3 properties take part of the performance optimization to avoid redoing the same work in all iterations
    private string $rootPath;
    private string $subPath;
    private string $directorySeparator = '/';

    /**
     * @throws \RuntimeException
     */
    public function __construct(string $path, int $flags, bool $ignoreUnreadableDirs = false)
    {}

    /**
     * Return an instance of SplFileInfo with support for relative paths.
     */
    public function current(): SplFileInfo
    {}

    public function hasChildren(bool $allowLinks = false): bool
    {}

    /**
     * @throws AccessDeniedException
     */
    public function getChildren(): \RecursiveDirectoryIterator
    {}

    public function next(): void
    {}

    public function rewind(): void
    {}
}
