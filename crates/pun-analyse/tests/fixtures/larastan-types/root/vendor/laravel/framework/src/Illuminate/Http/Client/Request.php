<?php

namespace Illuminate\Http\Client;

use ArrayAccess;
use Illuminate\Support\Collection;
use Illuminate\Support\Traits\Macroable;
use Illuminate\Support\Uri;
use LogicException;

class Request implements ArrayAccess
{
    use Macroable;

    /**
     * The underlying PSR request.
     *
     * @var \Psr\Http\Message\RequestInterface
     */
    protected $request;

    /**
     * The decoded payload for the request.
     *
     * @var array
     */
    protected $data;

    /**
     * The attribute data passed when building the PendingRequest.
     *
     * @var array<array-key, mixed>
     */
    protected $attributes = [];

    /**
     * Create a new request instance.
     *
     * @param  \Psr\Http\Message\RequestInterface  $request
     */
    public function __construct($request)
    {}

    /**
     * Get the request method.
     *
     * @return string
     */
    public function method()
    {}

    /**
     * Get the URL of the request.
     *
     * @return string
     */
    public function url()
    {}

    /**
     * Get the request URI as a URI instance.
     *
     * @return \Illuminate\Support\Uri
     */
    public function uri()
    {}

    /**
     * Determine if the request has a given header.
     *
     * @param  string  $key
     * @param  mixed  $value
     * @return bool
     */
    public function hasHeader($key, $value = null)
    {}

    /**
     * Determine if the request has the given headers.
     *
     * @param  array|string  $headers
     * @return bool
     */
    public function hasHeaders($headers)
    {}

    /**
     * Get the values for the header with the given name.
     *
     * @param  string  $key
     * @return array
     */
    public function header($key)
    {}

    /**
     * Get the request headers.
     *
     * @return array
     */
    public function headers()
    {}

    /**
     * Get the body of the request.
     *
     * @return string
     */
    public function body()
    {}

    /**
     * Determine if the request contains the given file.
     *
     * @param  string  $name
     * @param  string|null  $value
     * @param  string|null  $filename
     * @return bool
     */
    public function hasFile($name, $value = null, $filename = null)
    {}

    /**
     * Get the request's data (form parameters or JSON).
     *
     * @return array
     */
    public function data()
    {}

    /**
     * Get the request's form parameters.
     *
     * @return array
     */
    protected function parameters()
    {}

    /**
     * Get the decoded JSON body of the request.
     *
     * @return array
     */
    protected function json()
    {}

    /**
     * Determine if the request is simple form data.
     *
     * @return bool
     */
    public function isForm()
    {}

    /**
     * Determine if the request is JSON.
     *
     * @return bool
     */
    public function isJson()
    {}

    /**
     * Determine if the request is multipart.
     *
     * @return bool
     */
    public function isMultipart()
    {}

    /**
     * Set the decoded data on the request.
     *
     * @param  array  $data
     * @return $this
     */
    public function withData(array $data)
    {}

    /**
     * Get the attribute data from the request.
     *
     * @return array<array-key, mixed>
     */
    public function attributes()
    {}

    /**
     * Set the request's attribute data.
     *
     * @param  array<array-key, mixed>  $attributes
     * @return $this
     */
    public function setRequestAttributes($attributes)
    {}

    /**
     * Get the underlying PSR compliant request instance.
     *
     * @return \Psr\Http\Message\RequestInterface
     */
    public function toPsrRequest()
    {}

    /**
     * Determine if the given offset exists.
     *
     * @param  string  $offset
     * @return bool
     */
    public function offsetExists($offset): bool
    {}

    /**
     * Get the value for a given offset.
     *
     * @param  string  $offset
     * @return mixed
     */
    public function offsetGet($offset): mixed
    {}

    /**
     * Set the value at the given offset.
     *
     * @param  string  $offset
     * @param  mixed  $value
     * @return void
     *
     * @throws \LogicException
     */
    public function offsetSet($offset, $value): void
    {}

    /**
     * Unset the value at the given offset.
     *
     * @param  string  $offset
     * @return void
     *
     * @throws \LogicException
     */
    public function offsetUnset($offset): void
    {}
}
