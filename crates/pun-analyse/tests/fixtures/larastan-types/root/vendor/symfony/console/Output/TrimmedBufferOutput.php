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

use Symfony\Component\Console\Exception\InvalidArgumentException;
use Symfony\Component\Console\Formatter\OutputFormatterInterface;

/**
 * A BufferedOutput that keeps only the last N chars.
 *
 * @author Jérémy Derussé <jeremy@derusse.com>
 */
class TrimmedBufferOutput extends Output
{
    private int $maxLength;
    private string $buffer = '';

    public function __construct(int $maxLength, ?int $verbosity = self::VERBOSITY_NORMAL, bool $decorated = false, ?OutputFormatterInterface $formatter = null)
    {}

    /**
     * Empties buffer and returns its content.
     */
    public function fetch(): string
    {}

    protected function doWrite(string $message, bool $newline): void
    {}
}
