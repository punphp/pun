<?php

/*
 * This file is part of the Symfony package.
 *
 * (c) Fabien Potencier <fabien@symfony.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace Symfony\Component\Console\Input;

use Symfony\Component\Console\Exception\InvalidArgumentException;
use Symfony\Component\Console\Exception\LogicException;

/**
 * A InputDefinition represents a set of valid command line arguments and options.
 *
 * Usage:
 *
 *     $definition = new InputDefinition([
 *         new InputArgument('name', InputArgument::REQUIRED),
 *         new InputOption('foo', 'f', InputOption::VALUE_REQUIRED),
 *     ]);
 *
 * @author Fabien Potencier <fabien@symfony.com>
 */
class InputDefinition
{
    private array $arguments = [];
    private int $requiredCount = 0;
    private ?InputArgument $lastArrayArgument = null;
    private ?InputArgument $lastOptionalArgument = null;
    private array $options = [];
    private array $negations = [];
    private array $shortcuts = [];

    /**
     * @param array $definition An array of InputArgument and InputOption instance
     */
    public function __construct(array $definition = [])
    {}

    /**
     * Sets the definition of the input.
     */
    public function setDefinition(array $definition): void
    {}

    /**
     * Sets the InputArgument objects.
     *
     * @param InputArgument[] $arguments An array of InputArgument objects
     */
    public function setArguments(array $arguments = []): void
    {}

    /**
     * Adds an array of InputArgument objects.
     *
     * @param InputArgument[] $arguments An array of InputArgument objects
     */
    public function addArguments(?array $arguments = []): void
    {}

    /**
     * @throws LogicException When incorrect argument is given
     */
    public function addArgument(InputArgument $argument): void
    {}

    /**
     * Returns an InputArgument by name or by position.
     *
     * @throws InvalidArgumentException When argument given doesn't exist
     */
    public function getArgument(string|int $name): InputArgument
    {}

    /**
     * Returns true if an InputArgument object exists by name or position.
     */
    public function hasArgument(string|int $name): bool
    {}

    /**
     * Gets the array of InputArgument objects.
     *
     * @return InputArgument[]
     */
    public function getArguments(): array
    {}

    /**
     * Returns the number of InputArguments.
     */
    public function getArgumentCount(): int
    {}

    /**
     * Returns the number of required InputArguments.
     */
    public function getArgumentRequiredCount(): int
    {}

    /**
     * @return array<string|bool|int|float|array|null>
     */
    public function getArgumentDefaults(): array
    {}

    /**
     * Sets the InputOption objects.
     *
     * @param InputOption[] $options An array of InputOption objects
     */
    public function setOptions(array $options = []): void
    {}

    /**
     * Adds an array of InputOption objects.
     *
     * @param InputOption[] $options An array of InputOption objects
     */
    public function addOptions(array $options = []): void
    {}

    /**
     * @throws LogicException When option given already exist
     */
    public function addOption(InputOption $option): void
    {}

    /**
     * Returns an InputOption by name.
     *
     * @throws InvalidArgumentException When option given doesn't exist
     */
    public function getOption(string $name): InputOption
    {}

    /**
     * Returns true if an InputOption object exists by name.
     *
     * This method can't be used to check if the user included the option when
     * executing the command (use getOption() instead).
     */
    public function hasOption(string $name): bool
    {}

    /**
     * Gets the array of InputOption objects.
     *
     * @return InputOption[]
     */
    public function getOptions(): array
    {}

    /**
     * Returns true if an InputOption object exists by shortcut.
     */
    public function hasShortcut(string $name): bool
    {}

    /**
     * Returns true if an InputOption object exists by negated name.
     */
    public function hasNegation(string $name): bool
    {}

    /**
     * Gets an InputOption by shortcut.
     */
    public function getOptionForShortcut(string $shortcut): InputOption
    {}

    /**
     * @return array<string|bool|int|float|array|null>
     */
    public function getOptionDefaults(): array
    {}

    /**
     * Returns the InputOption name given a shortcut.
     *
     * @throws InvalidArgumentException When option given does not exist
     *
     * @internal
     */
    public function shortcutToName(string $shortcut): string
    {}

    /**
     * Returns the InputOption name given a negation.
     *
     * @throws InvalidArgumentException When option given does not exist
     *
     * @internal
     */
    public function negationToName(string $negation): string
    {}

    /**
     * Gets the synopsis.
     */
    public function getSynopsis(bool $short = false): string
    {}
}
