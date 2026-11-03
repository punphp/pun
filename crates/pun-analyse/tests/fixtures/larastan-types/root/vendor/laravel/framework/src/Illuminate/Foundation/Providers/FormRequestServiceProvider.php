<?php

namespace Illuminate\Foundation\Providers;

use Illuminate\Contracts\Validation\ValidatesWhenResolved;
use Illuminate\Foundation\Http\FormRequest;
use Illuminate\Routing\Redirector;
use Illuminate\Support\ServiceProvider;

class FormRequestServiceProvider extends ServiceProvider
{
    /**
     * Register the service provider.
     *
     * @return void
     */
    public function register()
    {}

    /**
     * Bootstrap the application services.
     *
     * @return void
     */
    public function boot()
    {}
}
