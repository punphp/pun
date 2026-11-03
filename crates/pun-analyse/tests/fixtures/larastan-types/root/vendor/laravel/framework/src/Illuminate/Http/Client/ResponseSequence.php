<?php

namespace Illuminate\Http\Client;

use Closure;
use Illuminate\Support\Traits\Macroable;
use OutOfBoundsException;

class ResponseSequence
{
    use Macroable;

    /**
     * The responses in the sequence.
     *
     * @var array
     */
    protected $responses;

    /**
     * Indicates that invoking this sequence when it is empty should throw an exception.
     *
     * @var bool
     */
    protected $failWhenEmpty = true;

    /**
     * The response that should be returned when the sequence is empty.
     *
     * @var \GuzzleHttp\Promise\PromiseInterface|\Closure|null
     */
    protected $emptyResponse;

    /**
     * Create a new response sequence.
     *
     * @param  array  $responses
     */
    public function __construct(array $responses)
    {}

    /**
     * Push a response to the sequence.
     *
     * @param  \Psr\Http\Message\StreamInterface|array|string|resource|null  $body
     * @param  int  $status
     * @param  array  $headers
     * @return $this
     */
    public function push($body = null, int $status = 200, array $headers = [])
    {}

    /**
     * Push a response with the given status code to the sequence.
     *
     * @param  int  $status
     * @param  array  $headers
     * @return $this
     */
    public function pushStatus(int $status, array $headers = [])
    {}

    /**
     * Push a response with the contents of a file as the body to the sequence.
     *
     * @param  string  $filePath
     * @param  int  $status
     * @param  array  $headers
     * @return $this
     */
    public function pushFile(string $filePath, int $status = 200, array $headers = [])
    {}

    /**
     * Push a connection exception to the sequence.
     *
     * @param  string|null  $message
     * @return $this
     */
    public function pushFailedConnection($message = null)
    {}

    /**
     * Push a response to the sequence.
     *
     * @param  mixed  $response
     * @return $this
     */
    public function pushResponse($response)
    {}

    /**
     * Make the sequence return a default response when it is empty.
     *
     * @param  \GuzzleHttp\Promise\PromiseInterface|\Closure  $response
     * @return $this
     */
    public function whenEmpty($response)
    {}

    /**
     * Make the sequence return a default response when it is empty.
     *
     * @return $this
     */
    public function dontFailWhenEmpty()
    {}

    /**
     * Indicate that this sequence has depleted all of its responses.
     *
     * @return bool
     */
    public function isEmpty()
    {}

    /**
     * Get the next response in the sequence.
     *
     * @param  \Illuminate\Http\Client\Request  $request
     * @return mixed
     *
     * @throws \OutOfBoundsException
     */
    public function __invoke($request)
    {}
}
