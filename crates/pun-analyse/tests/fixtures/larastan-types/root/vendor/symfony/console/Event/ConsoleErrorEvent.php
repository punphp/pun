<?php

/*
 * This file is part of the Symfony package.
 *
 * (c) Fabien Potencier <fabien@symfony.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace Symfony\Component\Console\Event;

use Symfony\Component\Console\Command\Command;
use Symfony\Component\Console\Input\InputInterface;
use Symfony\Component\Console\Output\OutputInterface;

/**
 * Allows to handle throwables thrown while running a command.
 *
 * @author Wouter de Jong <wouter@wouterj.nl>
 */
final class ConsoleErrorEvent extends ConsoleEvent
{
    private int $exitCode;

    public function __construct(
        InputInterface $input,
        OutputInterface $output,
        private \Throwable $error,
        ?Command $command = null,
    ) {}

    public function getError(): \Throwable
    {}

    public function setError(\Throwable $error): void
    {}

    public function setExitCode(int $exitCode): void
    {}

    public function getExitCode(): int
    {}
}
