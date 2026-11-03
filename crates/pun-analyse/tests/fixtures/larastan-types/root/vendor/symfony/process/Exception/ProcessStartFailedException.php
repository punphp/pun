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
 * Exception for processes failed during startup.
 */
class ProcessStartFailedException extends ProcessFailedException
{
    public function __construct(
        private Process $process,
        ?string $message,
    ) {}

    public function getProcess(): Process
    {}
}
