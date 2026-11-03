<?php

namespace Illuminate\Http\Middleware;

use Closure;
use Illuminate\Http\Request;

class TrustProxies
{
    /**
     * The trusted proxies for the application.
     *
     * @var array<int, string>|string|null
     */
    protected $proxies;

    /**
     * The trusted proxies headers for the application.
     *
     * @var int
     */
    protected $headers = Request::HEADER_X_FORWARDED_FOR |
        Request::HEADER_X_FORWARDED_HOST |
        Request::HEADER_X_FORWARDED_PORT |
        Request::HEADER_X_FORWARDED_PROTO |
        Request::HEADER_X_FORWARDED_PREFIX |
        Request::HEADER_X_FORWARDED_AWS_ELB;

    /**
     * The proxies that have been configured to always be trusted.
     *
     * @var array<int, string>|string|null
     */
    protected static $alwaysTrustProxies;

    /**
     * The proxies headers that have been configured to always be trusted.
     *
     * @var int|null
     */
    protected static $alwaysTrustHeaders;

    /**
     * Handle an incoming request.
     *
     * @param  \Illuminate\Http\Request  $request
     * @param  \Closure  $next
     * @return mixed
     *
     * @throws \Symfony\Component\HttpKernel\Exception\HttpException
     */
    public function handle(Request $request, Closure $next)
    {}

    /**
     * Sets the trusted proxies on the request.
     *
     * @param  \Illuminate\Http\Request  $request
     * @return void
     */
    protected function setTrustedProxyIpAddresses(Request $request)
    {}

    /**
     * Specify the IP addresses to trust explicitly.
     *
     * @param  \Illuminate\Http\Request  $request
     * @param  array  $trustedIps
     * @return void
     */
    protected function setTrustedProxyIpAddressesToSpecificIps(Request $request, array $trustedIps)
    {}

    /**
     * Set the trusted proxies to catchall addresses for IPv4 and IPv6.
     *
     * @param  \Illuminate\Http\Request  $request
     * @return void
     */
    protected function setTrustedProxyIpAddressesToTheCallingIp(Request $request)
    {}

    /**
     * Retrieve trusted header name(s), falling back to defaults if config not set.
     *
     * @return int A bit field of Request::HEADER_*, to set which headers to trust from your proxies.
     */
    protected function getTrustedHeaderNames()
    {}

    /**
     * Get the trusted headers.
     *
     * @return int
     */
    protected function headers()
    {}

    /**
     * Get the trusted proxies.
     *
     * @return array|string|null
     */
    protected function proxies()
    {}

    /**
     * Specify the IP addresses of proxies that should always be trusted.
     *
     * @param  array|string  $proxies
     * @return void
     */
    public static function at(array|string $proxies)
    {}

    /**
     * Specify the proxy headers that should always be trusted.
     *
     * @param  int  $headers
     * @return void
     */
    public static function withHeaders(int $headers)
    {}

    /**
     * Flush the state of the middleware.
     *
     * @return void
     */
    public static function flushState()
    {}
}
