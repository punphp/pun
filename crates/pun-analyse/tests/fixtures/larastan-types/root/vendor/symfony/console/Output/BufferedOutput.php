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

/**
 * @author Jean-François Simon <contact@jfsimon.fr>
 */
class BufferedOutput extends Output
{
    private string $buffer = '';

    /**
     * Empties buffer and returns its content.
     */
    public function fetch(): string
    {}

    protected function doWrite(string $message, bool $newline): void
    {}
}
