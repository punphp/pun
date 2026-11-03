<?php

namespace Illuminate\Support\Traits;

use Illuminate\Container\Container;

trait Localizable
{
    /**
     * Run the callback with the given locale.
     *
     * @template TReturn
     *
     * @param  string|null  $locale
     * @param  \Closure(): TReturn  $callback
     * @return TReturn
     */
    public function withLocale($locale, $callback)
    {}
}
