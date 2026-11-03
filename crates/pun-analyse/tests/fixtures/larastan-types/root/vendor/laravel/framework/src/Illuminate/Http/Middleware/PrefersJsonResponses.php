<?php

namespace Illuminate\Http\Middleware;

use Closure;

class PrefersJsonResponses
{
    /**
     * Handle an incoming request.
     *
     * @param  \Illuminate\Http\Request  $request
     * @param  \Closure  $next
     * @return mixed
     */
    public function handle($request, Closure $next)
    {}

    /**
     * Determine if the given "Accept" header value is broad enough to be treated as JSON.
     *
     * The header is broad when it's missing or every media-type listed is wildcard ("*\/*" or "application/*").
     *
     * @param  string|null  $accept
     * @return bool
     */
    protected function acceptHeaderIsBroad($accept)
    {}
}
