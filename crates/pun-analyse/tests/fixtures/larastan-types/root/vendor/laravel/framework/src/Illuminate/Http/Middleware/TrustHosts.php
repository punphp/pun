<?php

namespace Illuminate\Http\Middleware;

use Illuminate\Contracts\Foundation\Application;
use Illuminate\Http\Request;

class TrustHosts
{
    /**
     * The application instance.
     *
     * @var \Illuminate\Contracts\Foundation\Application
     */
    protected $app;

    /**
     * The trusted hosts that have been configured to always be trusted.
     *
     * @var array<int, string>|(callable(): array<int, string>)|null
     */
    protected static $alwaysTrust;

    /**
     * Indicates whether subdomains of the application URL should be trusted.
     *
     * @var bool|null
     */
    protected static $subdomains;

    /**
     * Create a new middleware instance.
     *
     * @param  \Illuminate\Contracts\Foundation\Application  $app
     */
    public function __construct(Application $app)
    {}

    /**
     * Get the host patterns that should be trusted.
     *
     * @return array
     */
    public function hosts()
    {}

    /**
     * Handle the incoming request.
     *
     * @param  \Illuminate\Http\Request  $request
     * @param  \Closure  $next
     * @return \Illuminate\Http\Response
     */
    public function handle(Request $request, $next)
    {}

    /**
     * Specify the hosts that should always be trusted.
     *
     * @param  array<int, string>|(callable(): array<int, string>)  $hosts
     * @param  bool  $subdomains
     * @return void
     */
    public static function at(array|callable $hosts, bool $subdomains = true)
    {}

    /**
     * Determine if the application should specify trusted hosts.
     *
     * @return bool
     */
    protected function shouldSpecifyTrustedHosts()
    {}

    /**
     * Get a regular expression matching the application URL and all of its subdomains.
     *
     * @return string|null
     */
    protected function allSubdomainsOfApplicationUrl()
    {}

    /**
     * Flush the state of the middleware.
     *
     * @return void
     */
    public static function flushState()
    {}
}
