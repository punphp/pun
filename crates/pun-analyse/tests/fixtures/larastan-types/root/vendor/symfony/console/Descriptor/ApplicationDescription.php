<?php

/*
 * This file is part of the Symfony package.
 *
 * (c) Fabien Potencier <fabien@symfony.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace Symfony\Component\Console\Descriptor;

use Symfony\Component\Console\Application;
use Symfony\Component\Console\Command\Command;
use Symfony\Component\Console\Exception\CommandNotFoundException;

/**
 * @author Jean-François Simon <jeanfrancois.simon@sensiolabs.com>
 *
 * @internal
 */
class ApplicationDescription
{
    public const GLOBAL_NAMESPACE = '_global';

    private array $namespaces;

    /**
     * @var array<string, Command>
     */
    private array $commands;

    /**
     * @var array<string, Command>
     */
    private array $aliases = [];

    public function __construct(
        private Application $application,
        private ?string $namespace = null,
        private bool $showHidden = false,
    ) {}

    public function getNamespaces(): array
    {}

    /**
     * @return Command[]
     */
    public function getCommands(): array
    {}

    /**
     * @throws CommandNotFoundException
     */
    public function getCommand(string $name): Command
    {}

    private function inspectApplication(): void
    {}

    /**
     * @return array<string, array<string, Command>>
     */
    private function sortCommands(array $commands): array
    {}
}
