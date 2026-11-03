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

use Symfony\Component\Console\Exception\RuntimeException;

/**
 * ArgvInput represents an input coming from the CLI arguments.
 *
 * Usage:
 *
 *     $input = new ArgvInput();
 *
 * By default, the `$_SERVER['argv']` array is used for the input values.
 *
 * This can be overridden by explicitly passing the input values in the constructor:
 *
 *     $input = new ArgvInput($_SERVER['argv']);
 *
 * If you pass it yourself, don't forget that the first element of the array
 * is the name of the running application.
 *
 * When passing an argument to the constructor, be sure that it respects
 * the same rules as the argv one. It's almost always better to use the
 * `StringInput` when you want to provide your own input.
 *
 * @author Fabien Potencier <fabien@symfony.com>
 *
 * @see http://www.gnu.org/software/libc/manual/html_node/Argument-Syntax.html
 * @see http://www.opengroup.org/onlinepubs/009695399/basedefs/xbd_chap12.html#tag_12_02
 */
class ArgvInput extends Input
{
    /** @var list<string> */
    private array $tokens;
    private array $parsed;

    /** @param list<string>|null $argv */
    public function __construct(?array $argv = null, ?InputDefinition $definition = null)
    {}

    /** @param list<string> $tokens */
    protected function setTokens(array $tokens): void
    {}

    protected function parse(): void
    {}

    protected function parseToken(string $token, bool $parseOptions): bool
    {}

    /**
     * Parses a short option.
     */
    private function parseShortOption(string $token): void
    {}

    /**
     * Parses a short option set.
     *
     * @throws RuntimeException When option given doesn't exist
     */
    private function parseShortOptionSet(string $name): void
    {}

    /**
     * Parses a long option.
     */
    private function parseLongOption(string $token): void
    {}

    /**
     * Parses an argument.
     *
     * @throws RuntimeException When too many arguments are given
     */
    private function parseArgument(string $token): void
    {}

    /**
     * Adds a short option value.
     *
     * @throws RuntimeException When option given doesn't exist
     */
    private function addShortOption(string $shortcut, mixed $value): void
    {}

    /**
     * Adds a long option value.
     *
     * @throws RuntimeException When option given doesn't exist
     */
    private function addLongOption(string $name, mixed $value): void
    {}

    public function getFirstArgument(): ?string
    {}

    private function getFirstArgumentPosition(): ?int
    {}

    public function hasParameterOption(string|array $values, bool $onlyParams = false): bool
    {}

    public function getParameterOption(string|array $values, string|bool|int|float|array|null $default = false, bool $onlyParams = false): mixed
    {}

    /**
     * Returns un-parsed and not validated tokens.
     *
     * @param bool $strip Whether to return the raw parameters (false) or the values after the command name (true)
     *
     * @return list<string>
     */
    public function getRawTokens(bool $strip = false): array
    {}

    /**
     * Returns a stringified representation of the args passed to the command.
     */
    public function __toString(): string
    {}
}
