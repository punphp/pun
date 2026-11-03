<?php

namespace Illuminate\Foundation\Exceptions\Renderer;

use Closure;
use Composer\Autoload\ClassLoader;
use Illuminate\Database\Eloquent\Model;
use Illuminate\Foundation\Bootstrap\HandleExceptions;
use Illuminate\Http\Request;
use Illuminate\Support\Collection;
use Symfony\Component\ErrorHandler\Exception\FlattenException;

class Exception
{
    /**
     * The "flattened" exception instance.
     *
     * @var \Symfony\Component\ErrorHandler\Exception\FlattenException
     */
    protected $exception;

    /**
     * The current request instance.
     *
     * @var \Illuminate\Http\Request
     */
    protected $request;

    /**
     * The exception listener instance.
     *
     * @var \Illuminate\Foundation\Exceptions\Renderer\Listener
     */
    protected $listener;

    /**
     * The application's base path.
     *
     * @var string
     */
    protected $basePath;

    /**
     * Creates a new exception renderer instance.
     *
     * @param  \Symfony\Component\ErrorHandler\Exception\FlattenException  $exception
     * @param  \Illuminate\Http\Request  $request
     * @param  \Illuminate\Foundation\Exceptions\Renderer\Listener  $listener
     * @param  string  $basePath
     */
    public function __construct(FlattenException $exception, Request $request, Listener $listener, string $basePath)
    {}

    /**
     * Get the exception title.
     *
     * @return string
     */
    public function title()
    {}

    /**
     * Get the exception message.
     *
     * @return string
     */
    public function message()
    {}

    /**
     * Get the exception class name.
     *
     * @return string
     */
    public function class()
    {}

    /**
     * Get the exception code.
     *
     * @return int|string
     */
    public function code()
    {}

    /**
     * Get the HTTP status code.
     *
     * @return int
     */
    public function httpStatusCode()
    {}

    /**
     * Get the previous exceptions in the chain.
     *
     * @return \Illuminate\Support\Collection<int, static>
     */
    public function previousExceptions()
    {}

    /**
     * Get the exception's frames.
     *
     * @return \Illuminate\Support\Collection<int, Frame>
     */
    public function frames()
    {}

    /**
     * Get the exception's frames grouped by vendor status.
     *
     * @return array<int, array{is_vendor: bool, frames: array<int, Frame>}>
     */
    public function frameGroups()
    {}

    /**
     * Get the exception's request instance.
     *
     * @return \Illuminate\Http\Request
     */
    public function request()
    {}

    /**
     * Get the request's headers.
     *
     * @return array<string, string>
     */
    public function requestHeaders()
    {}

    /**
     * Get the request's body parameters.
     *
     * @return string|null
     */
    public function requestBody()
    {}

    /**
     * Get the application's route context.
     *
     * @return array<string, string>
     */
    public function applicationRouteContext()
    {}

    /**
     * Get the application's route parameters context.
     *
     * @return string|null
     */
    public function applicationRouteParametersContext()
    {}

    /**
     * Get the application's SQL queries.
     *
     * @return array<int, array{connectionName: string, time: float, sql: string}>
     */
    public function applicationQueries()
    {}
}
