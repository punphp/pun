<?php

namespace Illuminate\Routing;

use Illuminate\Support\Arr;
use Illuminate\Support\Reflector;
use Illuminate\Support\Str;
use LogicException;
use UnexpectedValueException;

class RouteAction
{
    /**
     * Parse the given action into an array.
     *
     * @param  string  $uri
     * @param  mixed  $action
     * @return array
     */
    public static function parse($uri, $action)
    {}

    /**
     * Get an action for a route that has no action.
     *
     * @param  string  $uri
     * @return array
     *
     * @throws \LogicException
     */
    protected static function missingAction($uri)
    {}

    /**
     * Find the callable in an action array.
     *
     * @param  array  $action
     * @return callable
     */
    protected static function findCallable(array $action)
    {}

    /**
     * Make an action for an invokable controller.
     *
     * @param  string  $action
     * @return string
     *
     * @throws \UnexpectedValueException
     */
    protected static function makeInvokable($action)
    {}

    /**
     * Determine if the given array actions contain a serialized Closure.
     *
     * @param  array  $action
     * @return bool
     */
    public static function containsSerializedClosure(array $action)
    {}
}
