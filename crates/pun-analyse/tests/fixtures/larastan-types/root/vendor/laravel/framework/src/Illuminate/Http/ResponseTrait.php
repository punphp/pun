<?php

namespace Illuminate\Http;

use Illuminate\Http\Exceptions\HttpResponseException;
use Symfony\Component\HttpFoundation\HeaderBag;
use Throwable;

trait ResponseTrait
{
    /**
     * The original content of the response.
     *
     * @var mixed
     */
    public $original;

    /**
     * The exception that triggered the error response (if applicable).
     *
     * @var \Throwable|null
     */
    public $exception;

    /**
     * Get the status code for the response.
     *
     * @return int
     */
    public function status()
    {}

    /**
     * Get the status text for the response.
     *
     * @return string
     */
    public function statusText()
    {}

    /**
     * Get the content of the response.
     *
     * @return string
     */
    public function content()
    {}

    /**
     * Get the original response content.
     *
     * @return mixed
     */
    public function getOriginalContent()
    {}

    /**
     * Set a header on the Response.
     *
     * @param  string  $key
     * @param  array|string  $values
     * @param  bool  $replace
     * @return $this
     */
    public function header($key, $values, $replace = true)
    {}

    /**
     * Add an array of headers to the response.
     *
     * @param  \Symfony\Component\HttpFoundation\HeaderBag|array  $headers
     * @return $this
     */
    public function withHeaders($headers)
    {}

    /**
     * Remove a header(s) from the response.
     *
     * @param  array|string  $key
     * @return $this
     */
    public function withoutHeader($key)
    {}

    /**
     * Add a cookie to the response.
     *
     * @param  \Symfony\Component\HttpFoundation\Cookie|mixed  $cookie
     * @return $this
     */
    public function cookie($cookie)
    { func_get_args(); }

    /**
     * Add a cookie to the response.
     *
     * @param  \Symfony\Component\HttpFoundation\Cookie|mixed  $cookie
     * @return $this
     */
    public function withCookie($cookie)
    { func_get_args(); }

    /**
     * Add multiple cookies to the response.
     *
     * @param  array  $cookies
     * @return $this
     */
    public function withCookies(array $cookies)
    {}

    /**
     * Expire a cookie when sending the response.
     *
     * @param  \Symfony\Component\HttpFoundation\Cookie|mixed  $cookie
     * @param  string|null  $path
     * @param  string|null  $domain
     * @return $this
     */
    public function withoutCookie($cookie, $path = null, $domain = null)
    {}

    /**
     * Expire multiple cookies when sending the response.
     *
     * @param  array  $cookies
     * @param  string|null  $path
     * @param  string|null  $domain
     * @return $this
     */
    public function withoutCookies(array $cookies, $path = null, $domain = null)
    {}

    /**
     * Get the callback of the response.
     *
     * @return string|null
     */
    public function getCallback()
    {}

    /**
     * Set the exception to attach to the response.
     *
     * @param  \Throwable  $e
     * @return $this
     */
    public function withException(Throwable $e)
    {}

    /**
     * Throws the response in a HttpResponseException instance.
     *
     * @return never
     *
     * @throws \Illuminate\Http\Exceptions\HttpResponseException
     */
    public function throwResponse()
    {}
}
