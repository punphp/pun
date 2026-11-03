<?php

/*
 * This file is part of the Symfony package.
 *
 * (c) Fabien Potencier <fabien@symfony.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace Symfony\Component\Console\Command;

use Symfony\Component\Console\Attribute\AsCommand;
use Symfony\Component\Console\Input\InputArgument;
use Symfony\Component\Console\Input\InputInterface;
use Symfony\Component\Console\Input\InputOption;
use Symfony\Component\Console\Output\ConsoleOutputInterface;
use Symfony\Component\Console\Output\OutputInterface;
use Symfony\Component\Process\Process;

/**
 * Dumps the completion script for the current shell.
 *
 * @author Wouter de Jong <wouter@wouterj.nl>
 */
#[AsCommand(name: 'completion', description: 'Dump the shell completion script')]
final class DumpCompletionCommand extends Command
{
    private array $supportedShells;

    protected function configure(): void
    {}

    protected function execute(InputInterface $input, OutputInterface $output): int
    {}

    private static function guessShell(): string
    {}

    private function tailDebugLog(string $commandName, OutputInterface $output): void
    {}

    /**
     * @return string[]
     */
    private function getSupportedShells(): array
    {}
}
