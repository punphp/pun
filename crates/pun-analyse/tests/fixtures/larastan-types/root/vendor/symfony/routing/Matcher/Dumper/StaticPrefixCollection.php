<?php

/*
 * This file is part of the Symfony package.
 *
 * (c) Fabien Potencier <fabien@symfony.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace Symfony\Component\Routing\Matcher\Dumper;

use Symfony\Component\Routing\RouteCollection;

/**
 * Prefix tree of routes preserving routes order.
 *
 * @author Frank de Jonge <info@frankdejonge.nl>
 * @author Nicolas Grekas <p@tchwork.com>
 *
 * @internal
 */
class StaticPrefixCollection
{
    /**
     * @var string[]
     */
    private array $staticPrefixes = [];

    /**
     * @var string[]
     */
    private array $prefixes = [];

    /**
     * @var array[]|self[]
     */
    private array $items = [];

    public function __construct(
        private string $prefix = '/',
    ) {}

    public function getPrefix(): string
    {}

    /**
     * @return array[]|self[]
     */
    public function getRoutes(): array
    {}

    /**
     * Adds a route to a group.
     */
    public function addRoute(string $prefix, array|self $route): void
    {}

    /**
     * Linearizes back a set of nested routes into a collection.
     */
    public function populateCollection(RouteCollection $routes): RouteCollection
    {}

    /**
     * Gets the full and static common prefixes between two route patterns.
     *
     * The static prefix stops at last at the first opening bracket.
     */
    private function getCommonPrefix(string $prefix, string $anotherPrefix): array
    {}

    public static function handleError(int $type, string $msg): bool
    {}
}
