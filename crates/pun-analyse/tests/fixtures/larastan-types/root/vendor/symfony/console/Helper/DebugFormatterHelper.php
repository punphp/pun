<?php

/*
 * This file is part of the Symfony package.
 *
 * (c) Fabien Potencier <fabien@symfony.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace Symfony\Component\Console\Helper;

/**
 * Helps outputting debug information when running an external program from a command.
 *
 * An external program can be a Process, an HTTP request, or anything else.
 *
 * @author Fabien Potencier <fabien@symfony.com>
 */
class DebugFormatterHelper extends Helper
{
    private const COLORS = ['black', 'red', 'green', 'yellow', 'blue', 'magenta', 'cyan', 'white', 'default'];
    private array $started = [];
    private int $count = -1;

    /**
     * Starts a debug formatting session.
     */
    public function start(string $id, string $message, string $prefix = 'RUN'): string
    {}

    /**
     * Adds progress to a formatting session.
     */
    public function progress(string $id, string $buffer, bool $error = false, string $prefix = 'OUT', string $errorPrefix = 'ERR'): string
    {}

    /**
     * Stops a formatting session.
     */
    public function stop(string $id, string $message, bool $successful, string $prefix = 'RES'): string
    {}

    private function getBorder(string $id): string
    {}

    public function getName(): string
    {}
}
