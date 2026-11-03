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
 * CompiledRoutes are returned by the RouteCompiler class.
 *
 * @author Fabien Potencier <fabien@symfony.com>
 */
class CompiledRoute
{
    /**
     * @param string      $staticPrefix  The static prefix of the compiled route
     * @param string      $regex         The regular expression to use to match this route
     * @param array       $tokens        An array of tokens to use to generate URL for this route
     * @param array       $pathVariables An array of path variables
     * @param string|null $hostRegex     Host regex
     * @param array       $hostTokens    Host tokens
     * @param array       $hostVariables An array of host variables
     * @param array       $variables     An array of variables (variables defined in the path and in the host patterns)
     */
    public function __construct(
        private string $staticPrefix,
        private string $regex,
        private array $tokens,
        private array $pathVariables,
        private ?string $hostRegex = null,
        private array $hostTokens = [],
        private array $hostVariables = [],
        private array $variables = [],
    ) {}

    public function __serialize(): array
    {}

    public function __unserialize(array $data): void
    {}

    /**
     * Returns the static prefix.
     */
    public function getStaticPrefix(): string
    {}

    /**
     * Returns the regex.
     */
    public function getRegex(): string
    {}

    /**
     * Returns the host regex.
     */
    public function getHostRegex(): ?string
    {}

    /**
     * Returns the tokens.
     */
    public function getTokens(): array
    {}

    /**
     * Returns the host tokens.
     */
    public function getHostTokens(): array
    {}

    /**
     * Returns the variables.
     */
    public function getVariables(): array
    {}

    /**
     * Returns the path variables.
     */
    public function getPathVariables(): array
    {}

    /**
     * Returns the host variables.
     */
    public function getHostVariables(): array
    {}
}
