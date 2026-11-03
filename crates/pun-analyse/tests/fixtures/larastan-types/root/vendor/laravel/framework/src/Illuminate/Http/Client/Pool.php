<?php

namespace Illuminate\Http\Client;

use GuzzleHttp\Utils;

/**
 * @mixin \Illuminate\Http\Client\Factory
 */
class Pool
{
    /**
     * The factory instance.
     *
     * @var \Illuminate\Http\Client\Factory
     */
    protected $factory;

    /**
     * The handler function for the Guzzle client.
     *
     * @var callable
     */
    protected $handler;

    /**
     * The pool of requests.
     *
     * @var array<array-key, \Illuminate\Http\Client\PendingRequest>
     */
    protected $pool = [];

    /**
     * Create a new requests pool.
     *
     * @param  \Illuminate\Http\Client\Factory|null  $factory
     */
    public function __construct(?Factory $factory = null)
    {}

    /**
     * Add a request to the pool with a numeric index.
     *
     * @return \Illuminate\Http\Client\PendingRequest|\GuzzleHttp\Promise\Promise
     */
    public function newRequest()
    {}

    /**
     * Add a request to the pool with a key.
     *
     * @param  string  $key
     * @return \Illuminate\Http\Client\PendingRequest
     */
    public function as(string $key)
    {}

    /**
     * Retrieve a new async pending request.
     *
     * @return \Illuminate\Http\Client\PendingRequest
     */
    protected function asyncRequest()
    {}

    /**
     * Retrieve the requests in the pool.
     *
     * @return array<array-key, \Illuminate\Http\Client\PendingRequest>
     */
    public function getRequests()
    {}

    /**
     * Add a request to the pool with a numeric index and forward the method call to the request.
     *
     * @param  string  $method
     * @param  array  $parameters
     * @return \Illuminate\Http\Client\PendingRequest|\GuzzleHttp\Promise\Promise
     */
    public function __call($method, $parameters)
    {}
}
