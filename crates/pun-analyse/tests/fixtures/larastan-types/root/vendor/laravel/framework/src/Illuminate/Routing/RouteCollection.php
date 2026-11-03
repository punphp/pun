<?php

namespace Illuminate\Routing;

use Illuminate\Container\Container;
use Illuminate\Http\Request;

class RouteCollection extends AbstractRouteCollection
{
    /**
     * An array of the routes keyed by method.
     *
     * @var array
     */
    protected $routes = [];

    /**
     * Domain routes keyed by method, used to maintain domain-first ordering.
     *
     * @var array
     */
    protected $domainRoutes = [];

    /**
     * A flattened array of all of the routes.
     *
     * @var \Illuminate\Routing\Route[]
     */
    protected $allRoutes = [];

    /**
     * Domain routes in the flattened array, used to maintain domain-first ordering.
     *
     * @var \Illuminate\Routing\Route[]
     */
    protected $allDomainRoutes = [];

    /**
     * A look-up table of routes by their names.
     *
     * @var \Illuminate\Routing\Route[]
     */
    protected $nameList = [];

    /**
     * A look-up table of routes by controller action.
     *
     * @var \Illuminate\Routing\Route[]
     */
    protected $actionList = [];

    /**
     * Add a Route instance to the collection.
     *
     * @param  \Illuminate\Routing\Route  $route
     * @return \Illuminate\Routing\Route
     */
    public function add(Route $route)
    {}

    /**
     * Add the given route to the arrays of routes.
     *
     * @param  \Illuminate\Routing\Route  $route
     * @return void
     */
    protected function addToCollections($route)
    {}

    /**
     * Add the route to any look-up tables if necessary.
     *
     * @param  \Illuminate\Routing\Route  $route
     * @return void
     */
    protected function addLookups($route)
    {}

    /**
     * Add a route to the controller action dictionary.
     *
     * @param  array  $action
     * @param  \Illuminate\Routing\Route  $route
     * @return void
     */
    protected function addToActionList($action, $route)
    {}

    /**
     * Determine if the given controller is in the action lookup table.
     *
     * @param  string  $controller
     * @return bool
     */
    protected function inActionLookup($controller)
    {}

    /**
     * Determine if the given name is in the name lookup table.
     *
     * @param  string  $name
     * @return bool
     */
    protected function inNameLookup($name)
    {}

    /**
     * Refresh the name look-up table.
     *
     * This is done in case any names are fluently defined or if routes are overwritten.
     *
     * @return void
     */
    public function refreshNameLookups()
    {}

    /**
     * Refresh the action look-up table.
     *
     * This is done in case any actions are overwritten with new controllers.
     *
     * @return void
     */
    public function refreshActionLookups()
    {}

    /**
     * Find the first route matching a given request.
     *
     * @param  \Illuminate\Http\Request  $request
     * @return \Illuminate\Routing\Route
     *
     * @throws \Symfony\Component\HttpKernel\Exception\MethodNotAllowedHttpException
     * @throws \Symfony\Component\HttpKernel\Exception\NotFoundHttpException
     */
    public function match(Request $request)
    {}

    /**
     * Get routes from the collection by method.
     *
     * @param  string|null  $method
     * @return \Illuminate\Routing\Route[]
     */
    public function get($method = null)
    {}

    /**
     * Determine if the route collection contains a given named route.
     *
     * @param  string  $name
     * @return bool
     */
    public function hasNamedRoute($name)
    {}

    /**
     * Get a route instance by its name.
     *
     * @param  string  $name
     * @return \Illuminate\Routing\Route|null
     */
    public function getByName($name)
    {}

    /**
     * Get a route instance by its controller action.
     *
     * @param  string  $action
     * @return \Illuminate\Routing\Route|null
     */
    public function getByAction($action)
    {}

    /**
     * Get all of the routes in the collection.
     *
     * @return \Illuminate\Routing\Route[]
     */
    public function getRoutes()
    {}

    /**
     * Get all of the routes keyed by their HTTP verb / method.
     *
     * @return array
     */
    public function getRoutesByMethod()
    {}

    /**
     * Get all of the routes keyed by their name.
     *
     * @return \Illuminate\Routing\Route[]
     */
    public function getRoutesByName()
    {}

    /**
     * Convert the collection to a Symfony RouteCollection instance.
     *
     * @return \Symfony\Component\Routing\RouteCollection
     */
    public function toSymfonyRouteCollection()
    {}

    /**
     * Convert the collection to a CompiledRouteCollection instance.
     *
     * @param  \Illuminate\Routing\Router  $router
     * @param  \Illuminate\Container\Container  $container
     * @return \Illuminate\Routing\CompiledRouteCollection
     */
    public function toCompiledRouteCollection(Router $router, Container $container)
    {}
}
