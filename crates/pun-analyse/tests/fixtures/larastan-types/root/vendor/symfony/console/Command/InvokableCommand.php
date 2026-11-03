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
use Symfony\Component\Console\ArgumentResolver\ArgumentResolver;
use Symfony\Component\Console\ArgumentResolver\ArgumentResolverInterface;
use Symfony\Component\Console\Attribute\Argument;
use Symfony\Component\Console\Attribute\Interact;
use Symfony\Component\Console\Attribute\MapInput;
use Symfony\Component\Console\Attribute\Option;
use Symfony\Component\Console\Cursor;
use Symfony\Component\Console\Input\InputArgument;
use Symfony\Component\Console\Input\InputDefinition;
use Symfony\Component\Console\Input\InputInterface;
use Symfony\Component\Console\Input\RawInputInterface;
use Symfony\Component\Console\Interaction\Interaction;
use Symfony\Component\Console\Output\OutputInterface;
use Symfony\Component\Console\Style\SymfonyStyle;

/**
 * Represents an invokable command.
 *
 * @author Yonel Ceruto <open@yceruto.dev>
 *
 * @internal
 */
class InvokableCommand implements SignalableCommandInterface
{
    /**
     * @var list<array{?object, Command}>
     */
    private static array $running = [];

    private readonly ?SignalableCommandInterface $signalableCommand;
    private readonly \ReflectionFunction $invokable;
    /**
     * @var list<Interaction>|null
     */
    private ?array $interactions = null;
    private $code;

    public function __construct(
        private readonly Command $command,
        callable $code,
        private ?ArgumentResolverInterface $argumentResolver = null,
    ) {}

    /**
     * Invokes a callable with parameters generated from the input interface.
     */
    public function __invoke(InputInterface $input, OutputInterface $output): int
    {}

    /**
     * Configures the input definition from an invokable-defined function.
     *
     * Processes the parameters of the reflection function to extract and
     * add arguments or options to the provided input definition.
     */
    public function configure(InputDefinition $definition): void
    {}

    public function getCode(): callable
    {}

    /**
     * Returns the command that is currently running the given object, if any.
     */
    public static function getRunningCommand(object $code): ?Command
    {}

    private function getClosure(callable $code): \Closure
    {}

    private function getParameters(\ReflectionFunction $function, InputInterface $input, OutputInterface $output): array
    {}

    public function getSubscribedSignals(): array
    {}

    public function handleSignal(int $signal, int|false $previousExitCode = 0): int|false
    {}

    public function isInteractive(): bool
    {}

    public function interact(InputInterface $input, OutputInterface $output): void
    {}

    private function collectInteractions(): void
    {}
}
