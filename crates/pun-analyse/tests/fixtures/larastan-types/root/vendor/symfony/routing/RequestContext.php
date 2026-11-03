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

use Symfony\Component\HttpFoundation\Request;

/**
 * Holds information about the current request.
 *
 * This class implements a fluent interface.
 *
 * @author Fabien Potencier <fabien@symfony.com>
 * @author Tobias Schultze <http://tobion.de>
 */
class RequestContext
{
    private string $baseUrl;
    private string $pathInfo;
    private string $method;
    private string $host;
    private string $scheme;
    private int $httpPort;
    private int $httpsPort;
    private string $queryString;
    private array $parameters = [];

    public function __construct(string $baseUrl = '', string $method = 'GET', string $host = 'localhost', string $scheme = 'http', int $httpPort = 80, int $httpsPort = 443, string $path = '/', string $queryString = '', ?array $parameters = null)
    {}

    public static function fromUri(string $uri, string $host = 'localhost', string $scheme = 'http', int $httpPort = 80, int $httpsPort = 443): self
    {}

    /**
     * Updates the RequestContext information based on a HttpFoundation Request.
     *
     * @return $this
     */
    public function fromRequest(Request $request): static
    {}

    /**
     * Gets the base URL.
     */
    public function getBaseUrl(): string
    {}

    /**
     * Sets the base URL.
     *
     * @return $this
     */
    public function setBaseUrl(string $baseUrl): static
    {}

    /**
     * Gets the path info.
     */
    public function getPathInfo(): string
    {}

    /**
     * Sets the path info.
     *
     * @return $this
     */
    public function setPathInfo(string $pathInfo): static
    {}

    /**
     * Gets the HTTP method.
     *
     * The method is always an uppercased string.
     */
    public function getMethod(): string
    {}

    /**
     * Sets the HTTP method.
     *
     * @return $this
     */
    public function setMethod(string $method): static
    {}

    /**
     * Gets the HTTP host.
     *
     * The host is always lowercased because it must be treated case-insensitive.
     */
    public function getHost(): string
    {}

    /**
     * Sets the HTTP host.
     *
     * @return $this
     */
    public function setHost(string $host): static
    {}

    /**
     * Gets the HTTP scheme.
     */
    public function getScheme(): string
    {}

    /**
     * Sets the HTTP scheme.
     *
     * @return $this
     */
    public function setScheme(string $scheme): static
    {}

    /**
     * Gets the HTTP port.
     */
    public function getHttpPort(): int
    {}

    /**
     * Sets the HTTP port.
     *
     * @return $this
     */
    public function setHttpPort(int $httpPort): static
    {}

    /**
     * Gets the HTTPS port.
     */
    public function getHttpsPort(): int
    {}

    /**
     * Sets the HTTPS port.
     *
     * @return $this
     */
    public function setHttpsPort(int $httpsPort): static
    {}

    /**
     * Gets the query string without the "?".
     */
    public function getQueryString(): string
    {}

    /**
     * Sets the query string.
     *
     * @return $this
     */
    public function setQueryString(?string $queryString): static
    {}

    /**
     * Returns the parameters.
     */
    public function getParameters(): array
    {}

    /**
     * Sets the parameters.
     *
     * @param array $parameters The parameters
     *
     * @return $this
     */
    public function setParameters(array $parameters): static
    {}

    /**
     * Gets a parameter value.
     */
    public function getParameter(string $name): mixed
    {}

    /**
     * Checks if a parameter value is set for the given parameter.
     */
    public function hasParameter(string $name): bool
    {}

    /**
     * Sets a parameter value.
     *
     * @return $this
     */
    public function setParameter(string $name, mixed $parameter): static
    {}

    public function isSecure(): bool
    {}
}
