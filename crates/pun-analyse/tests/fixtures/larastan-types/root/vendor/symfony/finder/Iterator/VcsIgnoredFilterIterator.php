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

use Symfony\Component\Finder\Gitignore;

/**
 * @extends \FilterIterator<string, \SplFileInfo>
 */
final class VcsIgnoredFilterIterator extends \FilterIterator
{
    private string $searchDir;
    private string $realSearchDir;
    private string $baseDir;
    private ?string $gitInfoExcludeFile = null;

    /**
     * @var array<string, list<array{0: string, 1: bool, 2: bool}>|null>
     */
    private array $gitignoreFilesCache = [];

    /**
     * @var array<string, bool>
     */
    private array $ignoredPathsCache = [];

    /**
     * @param \Iterator<string, \SplFileInfo> $iterator
     */
    public function __construct(\Iterator $iterator, string $baseDir)
    {}

    public function accept(): bool
    {}

    private function isIgnored(string $path): bool
    {}

    /**
     * @return list<string>
     */
    private function parentDirectoriesUpwards(string $from): array
    {}

    private function parentDirectoriesUpTo(string $from, string $upTo): array
    {}

    /**
     * @return list<string>
     */
    private function parentDirectoriesDownwards(string $fileRealPath): array
    {}

    private function getGitInfoExcludePath(): ?string
    {}

    private function resolveGitPath(string $path, string $from): string
    {}

    /**
     * Returns the rules of a .gitignore file, last one first, as [regex, isNegated, isDirOnly] tuples.
     *
     * @return list<array{0: string, 1: bool, 2: bool}>|null
     */
    private function readGitignoreFile(string $path): ?array
    {}

    private function normalizePath(string $path): string
    {}
}
