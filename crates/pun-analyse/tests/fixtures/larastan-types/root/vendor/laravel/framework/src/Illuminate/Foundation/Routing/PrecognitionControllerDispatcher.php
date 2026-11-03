<?php

namespace Illuminate\Foundation\Routing;

use Illuminate\Routing\ControllerDispatcher;
use Illuminate\Routing\Route;
use RuntimeException;

class PrecognitionControllerDispatcher extends ControllerDispatcher
{
    /**
     * Dispatch a request to a given controller and method.
     *
     * @param  \Illuminate\Routing\Route  $route
     * @param  mixed  $controller
     * @param  string  $method
     * @return void
     */
    public function dispatch(Route $route, $controller, $method)
    {}

    /**
     * Ensure that the given method exists on the controller.
     *
     * @param  object  $controller
     * @param  string  $method
     * @return $this
     *
     * @throws \RuntimeException
     */
    protected function ensureMethodExists($controller, $method)
    {}
}
