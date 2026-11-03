<?php

/*
 * This file is part of the Symfony package.
 *
 * (c) Fabien Potencier <fabien@symfony.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace Symfony\Component\Routing\Matcher;

use Symfony\Component\ExpressionLanguage\ExpressionFunctionProviderInterface;
use Symfony\Component\ExpressionLanguage\ExpressionLanguage;
use Symfony\Component\HttpFoundation\Request;
use Symfony\Component\Routing\Exception\MethodNotAllowedException;
use Symfony\Component\Routing\Exception\NoConfigurationException;
use Symfony\Component\Routing\Exception\ResourceNotFoundException;
use Symfony\Component\Routing\RequestContext;
use Symfony\Component\Routing\Route;
use Symfony\Component\Routing\RouteCollection;

/**
 * UrlMatcher matches URL based on a set of routes.
 *
 * @author Fabien Potencier <fabien@symfony.com>
 */
class UrlMatcher implements UrlMatcherInterface, RequestMatcherInterface
{
    public const REQUIREMENT_MATCH = 0;
    public const REQUIREMENT_MISMATCH = 1;
    public const ROUTE_MATCH = 2;

    /**
     * Collects HTTP methods that would be allowed for the request.
     */
    protected array $allow = [];

    /**
     * Collects URI schemes that would be allowed for the request.
     *
     * @internal
     */
    protected array $allowSchemes = [];

    /**
     * Collects URI schemes that would be allowed for the request before a trailing slash redirection took precedence.
     *
     * @internal
     */
    protected array $trailingSlashSchemes = [];

    protected ?Request $request = null;
    protected ExpressionLanguage $expressionLanguage;

    /**
     * @var ExpressionFunctionProviderInterface[]
     */
    protected array $expressionLanguageProviders = [];

    public function __construct(
        protected RouteCollection $routes,
        protected RequestContext $context,
    ) {}

    public function setContext(RequestContext $context): void
    {}

    public function getContext(): RequestContext
    {}

    public function match(string $pathinfo): array
    {}

    public function matchRequest(Request $request): array
    {}

    public function addExpressionLanguageProvider(ExpressionFunctionProviderInterface $provider): void
    {}

    /**
     * Tries to match a URL with a set of routes.
     *
     * @param string $pathinfo The path info to be parsed
     *
     * @throws NoConfigurationException  If no routing configuration could be found
     * @throws ResourceNotFoundException If the resource could not be found
     * @throws MethodNotAllowedException If the resource was found but the request method is not allowed
     */
    protected function matchCollection(string $pathinfo, RouteCollection $routes): array
    {}

    /**
     * Returns an array of values to use as request attributes.
     *
     * As this method requires the Route object, it is not available
     * in matchers that do not have access to the matched Route instance
     * (like the PHP and Apache matcher dumpers).
     */
    protected function getAttributes(Route $route, string $name, array $attributes): array
    {}

    /**
     * Handles specific route requirements.
     *
     * @return array The first element represents the status, the second contains additional information
     */
    protected function handleRouteRequirements(string $pathinfo, string $name, Route $route, array $routeParameters): array
    {}

    /**
     * Get merged default parameters.
     */
    protected function mergeDefaults(array $params, array $defaults): array
    {}

    protected function getExpressionLanguage(): ExpressionLanguage
    {}

    /**
     * @internal
     */
    protected function createRequest(string $pathinfo): ?Request
    {}
}
