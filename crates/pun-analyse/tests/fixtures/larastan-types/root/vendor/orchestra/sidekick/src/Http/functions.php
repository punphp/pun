<?php

namespace Orchestra\Sidekick\Http;

if (! \function_exists('Orchestra\Sidekick\Http\safe_int')) {
    /**
     * Convert large id higher than Number.MAX_SAFE_INTEGER to string.
     *
     * https://stackoverflow.com/questions/47188449/json-max-int-number/47188576
     *
     * @api
     */
    function safe_int(mixed $value): mixed
    {}
}
