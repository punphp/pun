<?php

namespace Illuminate\Routing\Middleware;

use Closure;
use Illuminate\Routing\Exceptions\InvalidSignatureException;
use Illuminate\Support\Arr;

class ValidateSignature
{
    /**
     * The names of the parameters that should be ignored.
     *
     * @var array<int, string>
     */
    protected $ignore = [
        //
    ];

    /**
     * The globally ignored parameters.
     *
     * @var array
     */
    protected static $neverValidate = [];

    /**
     * Specify that the URL signature is for a relative URL.
     *
     * @param  array|string  $ignore
     * @return string
     */
    public static function relative($ignore = [])
    {}

    /**
     * Specify that the URL signature is for an absolute URL.
     *
     * @param  array|string  $ignore
     * @return class-string
     */
    public static function absolute($ignore = [])
    {}

    /**
     * Handle an incoming request.
     *
     * @param  \Illuminate\Http\Request  $request
     * @param  \Closure  $next
     * @param  array|null  $args
     * @return \Illuminate\Http\Response
     *
     * @throws \Illuminate\Routing\Exceptions\InvalidSignatureException
     */
    public function handle($request, Closure $next, ...$args)
    {}

    /**
     * Parse the additional arguments given to the middleware.
     *
     * @param  array  $args
     * @return array
     */
    protected function parseArguments(array $args)
    {}

    /**
     * Indicate that the given parameters should be ignored during signature validation.
     *
     * @param  array|string  $parameters
     * @return void
     */
    public static function except($parameters)
    {}
}
