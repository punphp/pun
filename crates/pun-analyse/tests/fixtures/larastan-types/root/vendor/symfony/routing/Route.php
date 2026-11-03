<?php

/*
 * This file is part of the Symfony package.
 *
 * (c) Fabien Potencier <fabien@symfony.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace Symfony\Component\Routing;

/**
 * A Route describes a route and its parameters.
 *
 * @author Fabien Potencier <fabien@symfony.com>
 * @author Tobias Schultze <http://tobion.de>
 */
class Route
{
    private string $path = '/';
    private string $host = '';
    private array $schemes = [];
    private array $methods = [];
    private array $defaults = [];
    private array $requirements = [];
    private array $options = [];
    private string $condition = '';
    private ?CompiledRoute $compiled = null;

    /**
     * Constructor.
     *
     * Available options:
     *
     *  * compiler_class: A class name able to compile this route instance (RouteCompiler by default)
     *  * utf8:           Whether UTF-8 matching is enforced or not
     *
     * @param string                    $path         The path pattern to match
     * @param array                     $defaults     An array of default parameter values
     * @param array<string|\Stringable> $requirements An array of requirements for parameters (regexes)
     * @param array                     $options      An array of options
     * @param string|null               $host         The host pattern to match
     * @param string|string[]           $schemes      A required URI scheme or an array of restricted schemes
     * @param string|string[]           $methods      A required HTTP method or an array of restricted methods
     * @param string|null               $condition    A condition that should evaluate to true for the route to match
     */
    public function __construct(string $path, array $defaults = [], array $requirements = [], array $options = [], ?string $host = '', string|array $schemes = [], string|array $methods = [], ?string $condition = '')
    {}

    public function __serialize(): array
    {}

    public function __unserialize(array $data): void
    {}

    public function getPath(): string
    {}

    /**
     * @return $this
     */
    public function setPath(string $pattern): static
    {}

    public function getHost(): string
    {}

    /**
     * @return $this
     */
    public function setHost(?string $pattern): static
    {}

    /**
     * Returns the lowercased schemes this route is restricted to.
     * So an empty array means that any scheme is allowed.
     *
     * @return string[]
     */
    public function getSchemes(): array
    {}

    /**
     * Sets the schemes (e.g. 'https') this route is restricted to.
     * So an empty array means that any scheme is allowed.
     *
     * @param string|string[] $schemes The scheme or an array of schemes
     *
     * @return $this
     */
    public function setSchemes(string|array $schemes): static
    {}

    /**
     * Checks if a scheme requirement has been set.
     */
    public function hasScheme(string $scheme): bool
    {}

    /**
     * Returns the uppercased HTTP methods this route is restricted to.
     * So an empty array means that any method is allowed.
     *
     * @return string[]
     */
    public function getMethods(): array
    {}

    /**
     * Sets the HTTP methods (e.g. 'POST') this route is restricted to.
     * So an empty array means that any method is allowed.
     *
     * @param string|string[] $methods The method or an array of methods
     *
     * @return $this
     */
    public function setMethods(string|array $methods): static
    {}

    public function getOptions(): array
    {}

    /**
     * @return $this
     */
    public function setOptions(array $options): static
    {}

    /**
     * @return $this
     */
    public function addOptions(array $options): static
    {}

    /**
     * Sets an option value.
     *
     * @return $this
     */
    public function setOption(string $name, mixed $value): static
    {}

    /**
     * Returns the option value or null when not found.
     */
    public function getOption(string $name): mixed
    {}

    public function hasOption(string $name): bool
    {}

    public function getDefaults(): array
    {}

    /**
     * @return $this
     */
    public function setDefaults(array $defaults): static
    {}

    /**
     * @return $this
     */
    public function addDefaults(array $defaults): static
    {}

    public function getDefault(string $name): mixed
    {}

    public function hasDefault(string $name): bool
    {}

    /**
     * @return $this
     */
    public function setDefault(string $name, mixed $default): static
    {}

    public function getRequirements(): array
    {}

    /**
     * @return $this
     */
    public function setRequirements(array $requirements): static
    {}

    /**
     * @return $this
     */
    public function addRequirements(array $requirements): static
    {}

    public function getRequirement(string $key): ?string
    {}

    public function hasRequirement(string $key): bool
    {}

    /**
     * @return $this
     */
    public function setRequirement(string $key, string $regex): static
    {}

    public function getCondition(): string
    {}

    /**
     * @return $this
     */
    public function setCondition(?string $condition): static
    {}

    /**
     * Compiles the route.
     *
     * @throws \LogicException If the Route cannot be compiled because the
     *                         path or host pattern is invalid
     *
     * @see RouteCompiler which is responsible for the compilation process
     */
    public function compile(): CompiledRoute
    {}

    private function extractInlineDefaultsAndRequirements(string $pattern): string
    {}

    private function sanitizeRequirement(string $key, string $regex): string
    {}

    private function isLocalized(): bool
    {}
}
