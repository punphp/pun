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
use Symfony\Component\Console\Exception\InvalidOptionException;

/**
 * ArrayInput represents an input provided as an array.
 *
 * Usage:
 *
 *     $input = new ArrayInput(['command' => 'foo:bar', 'foo' => 'bar', '--bar' => 'foobar']);
 *
 * @author Fabien Potencier <fabien@symfony.com>
 */
class ArrayInput extends Input
{
    public function __construct(
        private array $parameters,
        ?InputDefinition $definition = null,
    ) {}

    public function getFirstArgument(): ?string
    {}

    public function hasParameterOption(string|array $values, bool $onlyParams = false): bool
    {}

    public function getParameterOption(string|array $values, string|bool|int|float|array|null $default = false, bool $onlyParams = false): mixed
    {}

    /**
     * Returns a stringified representation of the args passed to the command.
     */
    public function __toString(): string
    {}

    protected function parse(): void
    {}

    /**
     * Adds a short option value.
     *
     * @throws InvalidOptionException When option given doesn't exist
     */
    private function addShortOption(string $shortcut, mixed $value): void
    {}

    /**
     * Adds a long option value.
     *
     * @throws InvalidOptionException When option given doesn't exist
     * @throws InvalidOptionException When a required value is missing
     */
    private function addLongOption(string $name, mixed $value): void
    {}

    /**
     * Adds an argument value.
     *
     * @throws InvalidArgumentException When argument given doesn't exist
     */
    private function addArgument(string|int $name, mixed $value): void
    {}
}
