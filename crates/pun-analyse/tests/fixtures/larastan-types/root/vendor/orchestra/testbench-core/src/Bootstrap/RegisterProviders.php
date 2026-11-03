<?php

namespace Orchestra\Testbench\Bootstrap;

class RegisterProviders extends \Illuminate\Foundation\Bootstrap\RegisterProviders
{
    /**
     * Merge additional providers for Testbench.
     *
     * @internal
     *
     * @template TProviders of array<int, class-string>
     *
     * @param  TProviders  $providers
     * @return TProviders
     */
    public static function mergeAdditionalProvidersForTestbench(array $providers): array
    {}
}
