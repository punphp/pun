<?php

namespace Illuminate\Foundation\Providers;

use Illuminate\Console\Events\CommandFinished;
use Illuminate\Console\Scheduling\Schedule;
use Illuminate\Contracts\Console\Kernel as ConsoleKernel;
use Illuminate\Contracts\Container\Container;
use Illuminate\Contracts\Events\Dispatcher;
use Illuminate\Contracts\Foundation\Application;
use Illuminate\Contracts\Foundation\ExceptionRenderer;
use Illuminate\Contracts\Foundation\MaintenanceMode as MaintenanceModeContract;
use Illuminate\Contracts\View\Factory;
use Illuminate\Database\ConnectionInterface;
use Illuminate\Database\Grammar;
use Illuminate\Foundation\Console\CliDumper;
use Illuminate\Foundation\Exceptions\Renderer\Listener;
use Illuminate\Foundation\Exceptions\Renderer\Mappers\BladeMapper;
use Illuminate\Foundation\Exceptions\Renderer\Renderer;
use Illuminate\Foundation\Http\HtmlDumper;
use Illuminate\Foundation\MaintenanceModeManager;
use Illuminate\Foundation\Precognition;
use Illuminate\Foundation\Vite;
use Illuminate\Http\Client\Factory as HttpFactory;
use Illuminate\Http\Request;
use Illuminate\Log\Events\MessageLogged;
use Illuminate\Queue\Events\JobAttempted;
use Illuminate\Support\AggregateServiceProvider;
use Illuminate\Support\Defer\DeferredCallbackCollection;
use Illuminate\Support\Facades\URL;
use Illuminate\Support\Uri;
use Illuminate\Testing\LoggedExceptionCollection;
use Illuminate\Testing\ParallelTestingServiceProvider;
use Illuminate\Validation\ValidationException;
use Symfony\Component\ErrorHandler\ErrorRenderer\HtmlErrorRenderer;
use Symfony\Component\VarDumper\Caster\StubCaster;
use Symfony\Component\VarDumper\Cloner\AbstractCloner;

class FoundationServiceProvider extends AggregateServiceProvider
{
    /**
     * The provider class names.
     *
     * @var array<int, class-string<\Illuminate\Support\ServiceProvider>>
     */
    protected $providers = [
        FormRequestServiceProvider::class,
        ParallelTestingServiceProvider::class,
    ];

    /**
     * The singletons to register into the container.
     *
     * @var array
     */
    public $singletons = [
        HttpFactory::class => HttpFactory::class,
        Vite::class => Vite::class,
    ];

    /**
     * Boot the service provider.
     *
     * @return void
     */
    public function boot()
    {}

    /**
     * Register the service provider.
     *
     * @return void
     */
    public function register()
    {}

    /**
     * Register the console schedule implementation.
     *
     * @return void
     */
    public function registerConsoleSchedule()
    {}

    /**
     * Register a var dumper (with source) to debug variables.
     *
     * @return void
     */
    public function registerDumper()
    {}

    /**
     * Register the "validate" macro on the request.
     *
     * @return void
     *
     * @throws \Illuminate\Validation\ValidationException
     */
    public function registerRequestValidation()
    {}

    /**
     * Register the "hasValidSignature" macro on the request.
     *
     * @return void
     */
    public function registerRequestSignatureValidation()
    {}

    /**
     * Register the URL resolver for the URI generator.
     *
     * @return void
     */
    protected function registerUriUrlGeneration()
    {}

    /**
     * Register the "defer" function termination handler.
     *
     * @return void
     */
    protected function registerDeferHandler()
    {}

    /**
     * Register an event listener to track logged exceptions.
     *
     * @return void
     */
    protected function registerExceptionTracking()
    {}

    /**
     * Register the exceptions renderer.
     *
     * @return void
     */
    protected function registerExceptionRenderer()
    {}

    /**
     * Register the maintenance mode manager service.
     *
     * @return void
     */
    public function registerMaintenanceModeManager()
    {}
}
