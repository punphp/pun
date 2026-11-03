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
 * Generic executable finder.
 *
 * @author Fabien Potencier <fabien@symfony.com>
 * @author Johannes M. Schmitt <schmittjoh@gmail.com>
 */
class ExecutableFinder
{
    private const CMD_BUILTINS = [
        'assoc', 'break', 'call', 'cd', 'chdir', 'cls', 'color', 'copy', 'date',
        'del', 'dir', 'echo', 'endlocal', 'erase', 'exit', 'for', 'ftype', 'goto',
        'help', 'if', 'label', 'md', 'mkdir', 'mklink', 'move', 'path', 'pause',
        'popd', 'prompt', 'pushd', 'rd', 'rem', 'ren', 'rename', 'rmdir', 'set',
        'setlocal', 'shift', 'start', 'time', 'title', 'type', 'ver', 'vol',
    ];

    private array $suffixes = [];

    /**
     * Replaces default suffixes of executable.
     */
    public function setSuffixes(array $suffixes): void
    {}

    /**
     * Adds new possible suffix to check for executable, including the dot (.).
     *
     *     $finder = new ExecutableFinder();
     *     $finder->addSuffix('.foo');
     */
    public function addSuffix(string $suffix): void
    {}

    /**
     * Finds an executable by name.
     *
     * @param string      $name      The executable name (without the extension)
     * @param string|null $default   The default to return if no executable is found
     * @param array       $extraDirs Additional dirs to check into
     */
    public function find(string $name, ?string $default = null, array $extraDirs = []): ?string
    {}
}
