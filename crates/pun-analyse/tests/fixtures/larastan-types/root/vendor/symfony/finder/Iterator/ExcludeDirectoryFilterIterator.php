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

use Symfony\Component\Finder\SplFileInfo;

/**
 * ExcludeDirectoryFilterIterator filters out directories.
 *
 * @author Fabien Potencier <fabien@symfony.com>
 *
 * @extends \FilterIterator<string, SplFileInfo>
 *
 * @implements \RecursiveIterator<string, SplFileInfo>
 */
class ExcludeDirectoryFilterIterator extends \FilterIterator implements \RecursiveIterator
{
    /** @var \Iterator<string, SplFileInfo> */
    private \Iterator $iterator;
    private bool $isRecursive;
    /** @var array<string, true> */
    private array $excludedDirs = [];
    private ?string $excludedPattern = null;
    /** @var list<callable(SplFileInfo):bool> */
    private array $pruneFilters = [];

    /**
     * @param \Iterator<string, SplFileInfo>          $iterator    The Iterator to filter
     * @param list<string|callable(SplFileInfo):bool> $directories An array of directories to exclude
     */
    public function __construct(\Iterator $iterator, array $directories)
    {}

    /**
     * Filters the iterator values.
     */
    public function accept(): bool
    {}

    public function hasChildren(): bool
    {}

    public function getChildren(): self
    {}
}
