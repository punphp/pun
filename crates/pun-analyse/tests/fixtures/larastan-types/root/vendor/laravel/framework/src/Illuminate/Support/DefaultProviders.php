<?php

namespace Illuminate\Support;

class DefaultProviders
{
    /**
     * The current providers.
     *
     * @var array<class-string>
     */
    protected $providers;

    /**
     * Create a new default provider collection.
     *
     * @param  array<class-string>|null  $providers
     */
    public function __construct(?array $providers = null)
    {}

    /**
     * Merge the given providers into the provider collection.
     *
     * @param  array<class-string>  $providers
     * @return static
     */
    public function merge(array $providers)
    {}

    /**
     * Replace the given providers with other providers.
     *
     * @param  array<class-string, class-string>  $replacements
     * @return static
     */
    public function replace(array $replacements)
    {}

    /**
     * Disable the given providers.
     *
     * @param  array<class-string>  $providers
     * @return static
     */
    public function except(array $providers)
    {}

    /**
     * Convert the provider collection to an array.
     *
     * @return array<class-string>
     */
    public function toArray()
    {}
}
