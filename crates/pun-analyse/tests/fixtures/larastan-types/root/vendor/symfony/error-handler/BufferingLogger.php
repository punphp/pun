<?php

/*
 * This file is part of the Symfony package.
 *
 * (c) Fabien Potencier <fabien@symfony.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace Symfony\Component\ErrorHandler;

use Psr\Log\AbstractLogger;

/**
 * A buffering logger that stacks logs for later.
 *
 * @author Nicolas Grekas <p@tchwork.com>
 */
class BufferingLogger extends AbstractLogger
{
    private array $logs = [];

    public function log($level, $message, array $context = []): void
    {}

    public function cleanLogs(): array
    {}

    public function __serialize(): array
    {}

    public function __unserialize(array $data): void
    {}

    public function __destruct()
    {}
}
