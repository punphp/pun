<?php

/*
 * This file is part of the Symfony package.
 *
 * (c) Fabien Potencier <fabien@symfony.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace Symfony\Component\Process;

/**
 * An executable finder specifically designed for the PHP executable.
 *
 * @author Fabien Potencier <fabien@symfony.com>
 * @author Johannes M. Schmitt <schmittjoh@gmail.com>
 */
class PhpExecutableFinder
{
    private ExecutableFinder $executableFinder;

    public function __construct()
    {}

    /**
     * Finds The PHP executable.
     */
    public function find(bool $includeArgs = true): string|false
    {}

    /**
     * Finds the PHP executable arguments.
     *
     * @return list<non-empty-string>
     */
    public function findArguments(): array
    {}
}
