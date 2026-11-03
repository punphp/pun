<?php

namespace Illuminate\Routing;

use Illuminate\Contracts\Routing\ResponseFactory;

class ViewController extends Controller
{
    /**
     * The response factory implementation.
     *
     * @var \Illuminate\Contracts\Routing\ResponseFactory
     */
    protected $response;

    /**
     * Create a new controller instance.
     *
     * @param  \Illuminate\Contracts\Routing\ResponseFactory  $response
     */
    public function __construct(ResponseFactory $response)
    {}

    /**
     * Invoke the controller method.
     *
     * @param  mixed  ...$args
     * @return \Illuminate\Http\Response
     */
    public function __invoke(...$args)
    {}

    /**
     * Execute an action on the controller.
     *
     * @param  string  $method
     * @param  array  $parameters
     * @return \Symfony\Component\HttpFoundation\Response
     */
    public function callAction($method, $parameters)
    {}
}
