<?php

/*
 * This file is part of the Symfony package.
 *
 * (c) Fabien Potencier <fabien@symfony.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace Symfony\Component\Console\Output;

use Symfony\Component\Console\Formatter\NullOutputFormatter;
use Symfony\Component\Console\Formatter\OutputFormatterInterface;

/**
 * NullOutput suppresses all output.
 *
 *     $output = new NullOutput();
 *
 * @author Fabien Potencier <fabien@symfony.com>
 * @author Tobias Schultze <http://tobion.de>
 */
class NullOutput implements OutputInterface
{
    private NullOutputFormatter $formatter;

    public function setFormatter(OutputFormatterInterface $formatter): void
    {}

    public function getFormatter(): OutputFormatterInterface
    {}

    public function setDecorated(bool $decorated): void
    {}

    public function isDecorated(): bool
    {}

    public function setVerbosity(int $level): void
    {}

    public function getVerbosity(): int
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

    public function writeln(string|iterable $messages, int $options = self::OUTPUT_NORMAL): void
    {}

    public function write(string|iterable $messages, bool $newline = false, int $options = self::OUTPUT_NORMAL): void
    {}
}
