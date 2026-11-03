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

use Symfony\Component\Console\Application;
use Symfony\Component\Console\Attribute\AsCommand;
use Symfony\Component\Console\Completion\CompletionInput;
use Symfony\Component\Console\Completion\CompletionSuggestions;
use Symfony\Component\Console\Completion\Output\BashCompletionOutput;
use Symfony\Component\Console\Completion\Output\CompletionOutputInterface;
use Symfony\Component\Console\Completion\Output\FishCompletionOutput;
use Symfony\Component\Console\Completion\Output\ZshCompletionOutput;
use Symfony\Component\Console\Exception\CommandNotFoundException;
use Symfony\Component\Console\Exception\ExceptionInterface;
use Symfony\Component\Console\Input\InputDefinition;
use Symfony\Component\Console\Input\InputInterface;
use Symfony\Component\Console\Input\InputOption;
use Symfony\Component\Console\Output\OutputInterface;

/**
 * Responsible for providing the values to the shell completion.
 *
 * @author Wouter de Jong <wouter@wouterj.nl>
 */
#[AsCommand(name: '|_complete', description: 'Internal command to provide shell completion suggestions')]
final class CompleteCommand extends Command
{
    public const COMPLETION_API_VERSION = '1';

    private array $completionOutputs;
    private ?InputDefinition $fullDefinition = null;
    private bool $isDebug = false;

    /**
     * @param array<string, class-string<CompletionOutputInterface>> $completionOutputs A list of additional completion outputs, with shell name as key and FQCN as value
     */
    public function __construct(array $completionOutputs = [])
    {}

    public function setApplication(?Application $application): void
    {}

    public function setDefinition(array|InputDefinition $definition): static
    {}

    /**
     * @internal
     */
    public function mergeApplicationDefinition(bool $mergeArgs = true): void
    {}

    public function getDefinition(): InputDefinition
    {}

    protected function configure(): void
    {}

    protected function initialize(InputInterface $input, OutputInterface $output): void
    {}

    protected function execute(InputInterface $input, OutputInterface $output): int
    {}

    private function createCompletionInput(InputInterface $input): CompletionInput
    {}

    private function findCommand(CompletionInput $completionInput): ?Command
    {}

    private function log($messages): void
    {}
}
