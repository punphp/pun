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
use Symfony\Component\Console\Exception\RuntimeException;

/**
 * Input is the base class for all concrete Input classes.
 *
 * Three concrete classes are provided by default:
 *
 *  * `ArgvInput`: The input comes from the CLI arguments (argv)
 *  * `StringInput`: The input is provided as a string
 *  * `ArrayInput`: The input is provided as an array
 *
 * @author Fabien Potencier <fabien@symfony.com>
 */
abstract class Input implements RawInputInterface, StreamableInputInterface
{
    protected InputDefinition $definition;
    /** @var resource */
    protected $stream;
    protected array $options = [];
    protected array $arguments = [];
    protected bool $interactive = true;

    public function __construct(?InputDefinition $definition = null)
    {}

    public function bind(InputDefinition $definition): void
    {}

    /**
     * Processes command line arguments.
     */
    abstract protected function parse(): void;

    public function validate(): void
    {}

    public function isInteractive(): bool
    {}

    public function setInteractive(bool $interactive): void
    {}

    public function getArguments(): array
    {}

    public function getArgument(string $name): mixed
    {}

    public function setArgument(string $name, mixed $value): void
    {}

    public function hasArgument(string $name): bool
    {}

    public function getOptions(): array
    {}

    public function getOption(string $name): mixed
    {}

    public function setOption(string $name, mixed $value): void
    {}

    public function hasOption(string $name): bool
    {}

    /**
     * Escapes a token through escapeshellarg if it contains unsafe chars.
     */
    public function escapeToken(string $token): string
    {}

    /**
     * @param resource $stream
     */
    public function setStream($stream): void
    {}

    /**
     * @return resource
     */
    public function getStream()
    {}

    public function getRawArguments(): array
    {}

    public function getRawOptions(): array
    {}

    public function unparse(?array $optionNames = null): array
    {}
}
