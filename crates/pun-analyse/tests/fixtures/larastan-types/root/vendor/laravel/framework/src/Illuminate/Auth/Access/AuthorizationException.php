<?php

namespace Illuminate\Auth\Access;

use Exception;
use Throwable;

class AuthorizationException extends Exception
{
    /**
     * The authorization response returned by the gate.
     *
     * @var \Illuminate\Auth\Access\Response
     */
    protected $response;

    /**
     * The HTTP response status code.
     *
     * @var int|null
     */
    protected $status;

    /**
     * Create a new authorization exception instance.
     *
     * @param  string|null  $message
     * @param  mixed  $code
     * @param  \Throwable|null  $previous
     */
    public function __construct($message = null, $code = null, ?Throwable $previous = null)
    {}

    /**
     * Get the response from the gate.
     *
     * @return \Illuminate\Auth\Access\Response
     */
    public function response()
    {}

    /**
     * Set the response from the gate.
     *
     * @param  \Illuminate\Auth\Access\Response  $response
     * @return $this
     */
    public function setResponse($response)
    {}

    /**
     * Set the HTTP response status code.
     *
     * @param  int|null  $status
     * @return $this
     */
    public function withStatus($status)
    {}

    /**
     * Set the HTTP response status code to 404.
     *
     * @return $this
     */
    public function asNotFound()
    {}

    /**
     * Determine if the HTTP status code has been set.
     *
     * @return bool
     */
    public function hasStatus()
    {}

    /**
     * Get the HTTP status code.
     *
     * @return int|null
     */
    public function status()
    {}

    /**
     * Create a deny response object from this exception.
     *
     * @return \Illuminate\Auth\Access\Response
     */
    public function toResponse()
    {}
}
