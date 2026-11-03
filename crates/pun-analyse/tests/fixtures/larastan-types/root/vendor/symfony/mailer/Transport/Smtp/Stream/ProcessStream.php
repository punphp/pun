<?php

/*
 * This file is part of the Symfony package.
 *
 * (c) Fabien Potencier <fabien@symfony.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace Symfony\Component\Mailer\Transport\Smtp\Stream;

use Symfony\Component\Mailer\Exception\TransportException;

/**
 * A stream supporting local processes.
 *
 * @author Fabien Potencier <fabien@symfony.com>
 * @author Chris Corbyn
 *
 * @internal
 */
final class ProcessStream extends AbstractStream
{
    private string $command;
    private bool $interactive = false;

    public function setCommand(string $command): void
    {}

    public function setInteractive(bool $interactive): void
    {}

    public function initialize(): void
    {}

    public function terminate(): void
    {}

    protected function getReadConnectionDescription(): string
    {}
}
