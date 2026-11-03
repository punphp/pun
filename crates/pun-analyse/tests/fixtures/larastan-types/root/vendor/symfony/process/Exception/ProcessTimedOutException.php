<?php

/*
 * This file is part of the Symfony package.
 *
 * (c) Fabien Potencier <fabien@symfony.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace Symfony\Component\Process\Exception;

use Symfony\Component\Process\Process;

/**
 * Exception that is thrown when a process times out.
 *
 * @author Johannes M. Schmitt <schmittjoh@gmail.com>
 */
class ProcessTimedOutException extends RuntimeException
{
    public const TYPE_GENERAL = 1;
    public const TYPE_IDLE = 2;

    public function __construct(
        private Process $process,
        private int $timeoutType,
    ) {}

    public function getProcess(): Process
    {}

    public function isGeneralTimeout(): bool
    {}

    public function isIdleTimeout(): bool
    {}

    public function getExceededTimeout(): ?float
    {}
}
