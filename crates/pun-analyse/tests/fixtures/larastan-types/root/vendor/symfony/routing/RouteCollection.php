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

use Symfony\Component\Config\Resource\ResourceInterface;
use Symfony\Component\Routing\Exception\InvalidArgumentException;
use Symfony\Component\Routing\Exception\RouteCircularReferenceException;

/**
 * A RouteCollection represents a set of Route instances.
 *
 * When adding a route at the end of the collection, an existing route
 * with the same name is removed first. So there can only be one route
 * with a given name.
 *
 * @author Fabien Potencier <fabien@symfony.com>
 * @author Tobias Schultze <http://tobion.de>
 *
 * @implements \IteratorAggregate<string, Route>
 */
class RouteCollection implements \IteratorAggregate, \Countable
{
    /**
     * @var array<string, Route>
     */
    private array $routes = [];

    /**
     * @var array<string, Alias>
     */
    private array $aliases = [];

    /**
     * @var array<string, ResourceInterface>
     */
    private array $resources = [];

    /**
     * @var array<string, int>
     */
    private array $priorities = [];

    public function __clone()
    {}

    /**
     * Gets the current RouteCollection as an Iterator that includes all routes.
     *
     * It implements \IteratorAggregate.
     *
     * @see all()
     *
     * @return \ArrayIterator<string, Route>
     */
    public function getIterator(): \ArrayIterator
    {}

    /**
     * Gets the number of Routes in this collection.
     */
    public function count(): int
    {}

    public function add(string $name, Route $route, int $priority = 0): void
    {}

    /**
     * Returns all routes in this collection.
     *
     * @return array<string, Route>
     */
    public function all(): array
    {}

    /**
     * Gets a route by name.
     */
    public function get(string $name): ?Route
    {}

    /**
     * Removes a route or an array of routes by name from the collection.
     *
     * @param string|string[] $name The route name or an array of route names
     */
    public function remove(string|array $name): void
    {}

    /**
     * Adds a route collection at the end of the current set by appending all
     * routes of the added collection.
     */
    public function addCollection(self $collection): void
    {}

    /**
     * Adds a prefix to the path of all child routes.
     */
    public function addPrefix(string $prefix, array $defaults = [], array $requirements = []): void
    {}

    /**
     * Adds a prefix to the name of all the routes within in the collection.
     */
    public function addNamePrefix(string $prefix): void
    {}

    /**
     * Sets the host pattern on all routes.
     */
    public function setHost(?string $pattern, array $defaults = [], array $requirements = []): void
    {}

    /**
     * Sets a condition on all routes.
     *
     * Existing conditions will be overridden.
     */
    public function setCondition(?string $condition): void
    {}

    /**
     * Adds defaults to all routes.
     *
     * An existing default value under the same name in a route will be overridden.
     */
    public function addDefaults(array $defaults): void
    {}

    /**
     * Adds requirements to all routes.
     *
     * An existing requirement under the same name in a route will be overridden.
     */
    public function addRequirements(array $requirements): void
    {}

    /**
     * Adds options to all routes.
     *
     * An existing option value under the same name in a route will be overridden.
     */
    public function addOptions(array $options): void
    {}

    /**
     * Sets the schemes (e.g. 'https') all child routes are restricted to.
     *
     * @param string|string[] $schemes The scheme or an array of schemes
     */
    public function setSchemes(string|array $schemes): void
    {}

    /**
     * Sets the HTTP methods (e.g. 'POST') all child routes are restricted to.
     *
     * @param string|string[] $methods The method or an array of methods
     */
    public function setMethods(string|array $methods): void
    {}

    /**
     * Returns an array of resources loaded to build this collection.
     *
     * @return ResourceInterface[]
     */
    public function getResources(): array
    {}

    /**
     * Adds a resource for this collection. If the resource already exists
     * it is not added.
     */
    public function addResource(ResourceInterface $resource): void
    {}

    /**
     * Sets an alias for an existing route.
     *
     * @param string $name  The alias to create
     * @param string $alias The route to alias
     *
     * @throws InvalidArgumentException if the alias is for itself
     */
    public function addAlias(string $name, string $alias): Alias
    {}

    /**
     * @return array<string, Alias>
     */
    public function getAliases(): array
    {}

    public function getAlias(string $name): ?Alias
    {}

    public function getPriority(string $name): ?int
    {}
}
