<?php

namespace Illuminate\Auth\Access;

use Illuminate\Contracts\Support\Arrayable;
use Stringable;

class Response implements Arrayable, Stringable
{
    /**
     * Indicates whether the response was allowed.
     *
     * @var bool
     */
    protected $allowed;

    /**
     * The response message.
     *
     * @var string|null
     */
    protected $message;

    /**
     * The response code.
     *
     * @var mixed
     */
    protected $code;

    /**
     * The HTTP response status code.
     *
     * @var int|null
     */
    protected $status;

    /**
     * Create a new response.
     *
     * @param  bool  $allowed
     * @param  string|null  $message
     * @param  mixed  $code
     */
    public function __construct($allowed, $message = '', $code = null)
    {}

    /**
     * Create a new "allow" Response.
     *
     * @param  string|null  $message
     * @param  mixed  $code
     * @return \Illuminate\Auth\Access\Response
     */
    public static function allow($message = null, $code = null)
    {}

    /**
     * Create a new "deny" Response.
     *
     * @param  string|null  $message
     * @param  mixed  $code
     * @return \Illuminate\Auth\Access\Response
     */
    public static function deny($message = null, $code = null)
    {}

    /**
     * Create a new "deny" Response with a HTTP status code.
     *
     * @param  int  $status
     * @param  string|null  $message
     * @param  mixed  $code
     * @return \Illuminate\Auth\Access\Response
     */
    public static function denyWithStatus($status, $message = null, $code = null)
    {}

    /**
     * Create a new "deny" Response with a 404 HTTP status code.
     *
     * @param  string|null  $message
     * @param  mixed  $code
     * @return \Illuminate\Auth\Access\Response
     */
    public static function denyAsNotFound($message = null, $code = null)
    {}

    /**
     * Determine if the response was allowed.
     *
     * @return bool
     */
    public function allowed()
    {}

    /**
     * Determine if the response was denied.
     *
     * @return bool
     */
    public function denied()
    {}

    /**
     * Get the response message.
     *
     * @return string|null
     */
    public function message()
    {}

    /**
     * Get the response code / reason.
     *
     * @return mixed
     */
    public function code()
    {}

    /**
     * Throw authorization exception if response was denied.
     *
     * @return \Illuminate\Auth\Access\Response
     *
     * @throws \Illuminate\Auth\Access\AuthorizationException
     */
    public function authorize()
    {}

    /**
     * Set the HTTP response status code.
     *
     * @param  null|int  $status
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
     * Get the HTTP status code.
     *
     * @return int|null
     */
    public function status()
    {}

    /**
     * Convert the response to an array.
     *
     * @return array
     */
    public function toArray()
    {}

    /**
     * Get the string representation of the message.
     *
     * @return string
     */
    public function __toString()
    {}
}
