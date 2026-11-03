<?php

namespace Illuminate\Routing;

use ArrayIterator;
use Countable;
use Illuminate\Http\Request;
use Illuminate\Http\Response;
use Illuminate\Support\Str;
use IteratorAggregate;
use LogicException;
use Symfony\Component\HttpKernel\Exception\MethodNotAllowedHttpException;
use Symfony\Component\HttpKernel\Exception\NotFoundHttpException;
use Symfony\Component\Routing\Matcher\Dumper\CompiledUrlMatcherDumper;
use Symfony\Component\Routing\RouteCollection as SymfonyRouteCollection;
use Traversable;

abstract class AbstractRouteCollection implements Countable, IteratorAggregate, RouteCollectionInterface
{
    /**
     * Handle the matched route.
     *
     * @param  \Illuminate\Http\Request  $request
     * @param  \Illuminate\Routing\Route|null  $route
     * @return \Illuminate\Routing\Route
     *
     * @throws \Symfony\Component\HttpKernel\Exception\NotFoundHttpException
     */
    protected function handleMatchedRoute(Request $request, $route)
    {}

    /**
     * Determine if any routes match on another HTTP verb.
     *
     * @param  \Illuminate\Http\Request  $request
     * @return array
     */
    protected function checkForAlternateVerbs($request)
    {}

    /**
     * Determine if a route in the array matches the request.
     *
     * @param  \Illuminate\Routing\Route[]  $routes
     * @param  \Illuminate\Http\Request  $request
     * @param  bool  $includingMethod
     * @return \Illuminate\Routing\Route|null
     */
    protected function matchAgainstRoutes(array $routes, $request, $includingMethod = true)
    {}

    /**
     * Get a route (if necessary) that responds when other available methods are present.
     *
     * @param  \Illuminate\Http\Request  $request
     * @param  string[]  $methods
     * @return \Illuminate\Routing\Route
     *
     * @throws \Symfony\Component\HttpKernel\Exception\MethodNotAllowedHttpException
     */
    protected function getRouteForMethods($request, array $methods)
    {}

    /**
     * Throw a method not allowed HTTP exception.
     *
     * @param  \Illuminate\Http\Request  $request
     * @param  array  $others
     * @param  string  $method
     * @return never
     *
     * @throws \Symfony\Component\HttpKernel\Exception\MethodNotAllowedHttpException
     */
    protected function requestMethodNotAllowed($request, array $others, $method)
    {}

    /**
     * Throw a method not allowed HTTP exception.
     *
     * @param  array  $others
     * @param  string  $method
     * @return void
     *
     * @deprecated use requestMethodNotAllowed
     *
     * @throws \Symfony\Component\HttpKernel\Exception\MethodNotAllowedHttpException
     */
    protected function methodNotAllowed(array $others, $method)
    {}

    /**
     * Compile the routes for caching.
     *
     * @return array
     */
    public function compile()
    {}

    /**
     * Return the CompiledUrlMatcherDumper instance for the route collection.
     *
     * @return \Symfony\Component\Routing\Matcher\Dumper\CompiledUrlMatcherDumper
     */
    public function dumper()
    {}

    /**
     * Convert the collection to a Symfony RouteCollection instance.
     *
     * @return \Symfony\Component\Routing\RouteCollection
     */
    public function toSymfonyRouteCollection()
    {}

    /**
     * Add a route to the SymfonyRouteCollection instance.
     *
     * @param  \Symfony\Component\Routing\RouteCollection  $symfonyRoutes
     * @param  \Illuminate\Routing\Route  $route
     * @return \Symfony\Component\Routing\RouteCollection
     *
     * @throws \LogicException
     */
    protected function addToSymfonyRoutesCollection(SymfonyRouteCollection $symfonyRoutes, Route $route)
    {}

    /**
     * Get a randomly generated route name.
     *
     * @return string
     */
    protected function generateRouteName()
    {}

    /**
     * Get an iterator for the items.
     *
     * @return \ArrayIterator
     */
    public function getIterator(): Traversable
    {}

    /**
     * Count the number of items in the collection.
     *
     * @return int
     */
    public function count(): int
    {}
}
