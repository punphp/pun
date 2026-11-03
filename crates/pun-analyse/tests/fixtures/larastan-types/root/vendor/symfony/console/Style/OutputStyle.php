<?php

/*
 * This file is part of the Symfony package.
 *
 * (c) Fabien Potencier <fabien@symfony.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace Symfony\Component\Console\Style;

use Symfony\Component\Console\Formatter\OutputFormatterInterface;
use Symfony\Component\Console\Helper\ProgressBar;
use Symfony\Component\Console\Output\ConsoleOutputInterface;
use Symfony\Component\Console\Output\OutputInterface;

/**
 * Decorates output to add console style guide helpers.
 *
 * @author Kevin Bond <kevinbond@gmail.com>
 */
abstract class OutputStyle implements OutputInterface, StyleInterface
{
    public function __construct(
        private OutputInterface $output,
    ) {}

    public function newLine(int $count = 1): void
    {}

    public function createProgressBar(int $max = 0): ProgressBar
    {}

    public function write(string|iterable $messages, bool $newline = false, int $type = self::OUTPUT_NORMAL): void
    {}

    public function writeln(string|iterable $messages, int $type = self::OUTPUT_NORMAL): void
    {}

    public function setVerbosity(int $level): void
    {}

    public function getVerbosity(): int
    {}

    public function setDecorated(bool $decorated): void
    {}

    public function isDecorated(): bool
    {}

    public function setFormatter(OutputFormatterInterface $formatter): void
    {}

    public function getFormatter(): OutputFormatterInterface
    {}

    public function isSilent(): bool
    {}

    public function isQuiet(): bool
    {}

    public function isVerbose(): bool
    {}

    public function isVeryVerbose(): bool
    {}

    public function isDebug(): bool
    {}

    protected function getErrorOutput(): OutputInterface
    {}
}
