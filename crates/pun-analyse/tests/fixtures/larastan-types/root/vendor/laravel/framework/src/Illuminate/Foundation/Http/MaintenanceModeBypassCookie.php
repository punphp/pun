<?php

namespace Illuminate\Foundation\Http;

use Illuminate\Support\Carbon;
use Symfony\Component\HttpFoundation\Cookie;

class MaintenanceModeBypassCookie
{
    /**
     * Create a new maintenance mode bypass cookie.
     *
     * @param  string  $key
     * @return \Symfony\Component\HttpFoundation\Cookie
     */
    public static function create(string $key)
    {}

    /**
     * Determine if the given maintenance mode bypass cookie is valid.
     *
     * @param  string  $cookie
     * @param  string  $key
     * @return bool
     */
    public static function isValid(string $cookie, string $key)
    {}
}
