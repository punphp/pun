<?php

declare(strict_types=1);

/*
 * This file is part of the league/config package.
 *
 * (c) Colin O'Dell <colinodell@gmail.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace League\Config;

use Dflydev\DotAccessData\Data;
use Dflydev\DotAccessData\DataInterface;
use Dflydev\DotAccessData\Exception\DataException;
use Dflydev\DotAccessData\Exception\InvalidPathException;
use Dflydev\DotAccessData\Exception\MissingPathException;
use League\Config\Exception\UnknownOptionException;
use League\Config\Exception\ValidationException;
use Nette\Schema\Expect;
use Nette\Schema\Processor;
use Nette\Schema\Schema;
use Nette\Schema\ValidationException as NetteValidationException;

final class Configuration implements ConfigurationBuilderInterface, ConfigurationInterface
{
    /** @psalm-readonly */
    private Data $userConfig;

    /**
     * @var array<string, Schema>
     *
     * @psalm-allow-private-mutation
     */
    private array $configSchemas = [];

    /** @psalm-allow-private-mutation */
    private Data $finalConfig;

    /**
     * @var array<string, mixed>
     *
     * @psalm-allow-private-mutation
     */
    private array $cache = [];

    /** @psalm-readonly */
    private ConfigurationInterface $reader;

    /**
     * @param array<string, Schema> $baseSchemas
     */
    public function __construct(array $baseSchemas = [])
    {}

    /**
     * Registers a new configuration schema at the given top-level key
     *
     * @psalm-allow-private-mutation
     */
    public function addSchema(string $key, Schema $schema): void
    {}

    /**
     * {@inheritDoc}
     *
     * @psalm-allow-private-mutation
     */
    public function merge(array $config = []): void
    {}

    /**
     * {@inheritDoc}
     *
     * @psalm-allow-private-mutation
     */
    public function set(string $key, $value): void
    {}

    /**
     * {@inheritDoc}
     *
     * @psalm-external-mutation-free
     */
    public function get(string $key)
    {}

    /**
     * {@inheritDoc}
     *
     * @psalm-external-mutation-free
     */
    public function exists(string $key): bool
    {}

    /**
     * @psalm-mutation-free
     */
    public function reader(): ConfigurationInterface
    {}

    /**
     * @psalm-external-mutation-free
     */
    private function invalidate(): void
    {}

    /**
     * Applies the schema against the configuration to return the final configuration
     *
     * @throws ValidationException|UnknownOptionException|InvalidPathException
     *
     * @psalm-allow-private-mutation
     */
    private function build(string $topLevelKey): void
    {}

    /**
     * Recursively converts stdClass instances to arrays
     *
     * @phpstan-template T
     *
     * @param T $data
     *
     * @return mixed
     *
     * @phpstan-return ($data is \stdClass ? array<string, mixed> : T)
     *
     * @psalm-pure
     */
    private static function convertStdClassesToArrays($data)
    {}

    /**
     * @param string[] $warnings
     */
    private function raiseAnyDeprecationNotices(array $warnings): void
    {}

    /**
     * @throws InvalidPathException
     */
    private static function getTopLevelKey(string $path): string
    {}
}
