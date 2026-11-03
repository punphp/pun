<?php

namespace Illuminate\View;

use Illuminate\Contracts\Container\Container;
use Illuminate\Contracts\Events\Dispatcher;
use Illuminate\Contracts\Support\Arrayable;
use Illuminate\Contracts\View\Factory as FactoryContract;
use Illuminate\Support\Arr;
use Illuminate\Support\Traits\Macroable;
use Illuminate\View\Engines\EngineResolver;
use InvalidArgumentException;

class Factory implements FactoryContract
{
    use Macroable,
        Concerns\ManagesComponents,
        Concerns\ManagesEvents,
        Concerns\ManagesFragments,
        Concerns\ManagesLayouts,
        Concerns\ManagesLoops,
        Concerns\ManagesStacks,
        Concerns\ManagesTranslations;

    /**
     * The engine implementation.
     *
     * @var \Illuminate\View\Engines\EngineResolver
     */
    protected $engines;

    /**
     * The view finder implementation.
     *
     * @var \Illuminate\View\ViewFinderInterface
     */
    protected $finder;

    /**
     * The event dispatcher instance.
     *
     * @var \Illuminate\Contracts\Events\Dispatcher
     */
    protected $events;

    /**
     * The IoC container instance.
     *
     * @var \Illuminate\Contracts\Container\Container
     */
    protected $container;

    /**
     * Data that should be available to all templates.
     *
     * @var array
     */
    protected $shared = [];

    /**
     * The extension to engine bindings.
     *
     * @var array
     */
    protected $extensions = [
        'blade.php' => 'blade',
        'php' => 'php',
        'css' => 'file',
        'html' => 'file',
    ];

    /**
     * The view composer events.
     *
     * @var array
     */
    protected $composers = [];

    /**
     * The number of active rendering operations.
     *
     * @var int
     */
    protected $renderCount = 0;

    /**
     * The "once" block IDs that have been rendered.
     *
     * @var array
     */
    protected $renderedOnce = [];

    /**
     * The cached array of engines for paths.
     *
     * @var array
     */
    protected $pathEngineCache = [];

    /**
     * The cache of normalized names for views.
     *
     * @var array
     */
    protected $normalizedNameCache = [];

    /**
     * Create a new view factory instance.
     *
     * @param  \Illuminate\View\Engines\EngineResolver  $engines
     * @param  \Illuminate\View\ViewFinderInterface  $finder
     * @param  \Illuminate\Contracts\Events\Dispatcher  $events
     */
    public function __construct(EngineResolver $engines, ViewFinderInterface $finder, Dispatcher $events)
    {}

    /**
     * Get the evaluated view contents for the given view.
     *
     * @param  string  $path
     * @param  \Illuminate\Contracts\Support\Arrayable|array  $data
     * @param  array  $mergeData
     * @return \Illuminate\Contracts\View\View
     */
    public function file($path, $data = [], $mergeData = [])
    {}

    /**
     * Get the evaluated view contents for the given view.
     *
     * @param  string  $view
     * @param  \Illuminate\Contracts\Support\Arrayable|array  $data
     * @param  array  $mergeData
     * @return \Illuminate\Contracts\View\View
     */
    public function make($view, $data = [], $mergeData = [])
    {}

    /**
     * Get the first view that actually exists from the given list.
     *
     * @param  array  $views
     * @param  \Illuminate\Contracts\Support\Arrayable|array  $data
     * @param  array  $mergeData
     * @return \Illuminate\Contracts\View\View
     *
     * @throws \InvalidArgumentException
     */
    public function first(array $views, $data = [], $mergeData = [])
    {}

    /**
     * Get the rendered content of the view based on a given condition.
     *
     * @param  bool  $condition
     * @param  string  $view
     * @param  \Illuminate\Contracts\Support\Arrayable|array  $data
     * @param  array  $mergeData
     * @return string
     */
    public function renderWhen($condition, $view, $data = [], $mergeData = [])
    {}

    /**
     * Get the rendered content of the view based on the negation of a given condition.
     *
     * @param  bool  $condition
     * @param  string  $view
     * @param  \Illuminate\Contracts\Support\Arrayable|array  $data
     * @param  array  $mergeData
     * @return string
     */
    public function renderUnless($condition, $view, $data = [], $mergeData = [])
    {}

    /**
     * Get the rendered contents of a partial from a loop.
     *
     * @param  string  $view
     * @param  array  $data
     * @param  string  $iterator
     * @param  string  $empty
     * @return string
     */
    public function renderEach($view, $data, $iterator, $empty = 'raw|')
    {}

    /**
     * Normalize a view name.
     *
     * @param  string  $name
     * @return string
     */
    protected function normalizeName($name)
    {}

    /**
     * Parse the given data into a raw array.
     *
     * @param  mixed  $data
     * @return array
     */
    protected function parseData($data)
    {}

    /**
     * Create a new view instance from the given arguments.
     *
     * @param  string  $view
     * @param  string  $path
     * @param  \Illuminate\Contracts\Support\Arrayable|array  $data
     * @return \Illuminate\Contracts\View\View
     */
    protected function viewInstance($view, $path, $data)
    {}

    /**
     * Determine if a given view exists.
     *
     * @param  string  $view
     * @return bool
     */
    public function exists($view)
    {}

    /**
     * Get the appropriate view engine for the given path.
     *
     * @param  string  $path
     * @return \Illuminate\Contracts\View\Engine
     *
     * @throws \InvalidArgumentException
     */
    public function getEngineFromPath($path)
    {}

    /**
     * Get the extension used by the view file.
     *
     * @param  string  $path
     * @return string|null
     */
    protected function getExtension($path)
    {}

    /**
     * Add a piece of shared data to the environment.
     *
     * @param  array|string  $key
     * @param  mixed  $value
     * @return mixed
     */
    public function share($key, $value = null)
    {}

    /**
     * Increment the rendering counter.
     *
     * @return void
     */
    public function incrementRender()
    {}

    /**
     * Decrement the rendering counter.
     *
     * @return void
     */
    public function decrementRender()
    {}

    /**
     * Check if there are no active render operations.
     *
     * @return bool
     */
    public function doneRendering()
    {}

    /**
     * Determine if the given once token has been rendered.
     *
     * @param  string  $id
     * @return bool
     */
    public function hasRenderedOnce(string $id)
    {}

    /**
     * Mark the given once token as having been rendered.
     *
     * @param  string  $id
     * @return void
     */
    public function markAsRenderedOnce(string $id)
    {}

    /**
     * Add a location to the array of view locations.
     *
     * @param  string  $location
     * @return void
     */
    public function addLocation($location)
    {}

    /**
     * Prepend a location to the array of view locations.
     *
     * @param  string  $location
     * @return void
     */
    public function prependLocation($location)
    {}

    /**
     * Add a new namespace to the loader.
     *
     * @param  string  $namespace
     * @param  string|array  $hints
     * @return $this
     */
    public function addNamespace($namespace, $hints)
    {}

    /**
     * Prepend a new namespace to the loader.
     *
     * @param  string  $namespace
     * @param  string|array  $hints
     * @return $this
     */
    public function prependNamespace($namespace, $hints)
    {}

    /**
     * Replace the namespace hints for the given namespace.
     *
     * @param  string  $namespace
     * @param  string|array  $hints
     * @return $this
     */
    public function replaceNamespace($namespace, $hints)
    {}

    /**
     * Register a valid view extension and its engine.
     *
     * @param  string  $extension
     * @param  string  $engine
     * @param  \Closure|null  $resolver
     * @return void
     */
    public function addExtension($extension, $engine, $resolver = null)
    {}

    /**
     * Flush all of the factory state like sections and stacks.
     *
     * @return void
     */
    public function flushState()
    {}

    /**
     * Flush all of the section contents if done rendering.
     *
     * @return void
     */
    public function flushStateIfDoneRendering()
    {}

    /**
     * Get the extension to engine bindings.
     *
     * @return array
     */
    public function getExtensions()
    {}

    /**
     * Get the engine resolver instance.
     *
     * @return \Illuminate\View\Engines\EngineResolver
     */
    public function getEngineResolver()
    {}

    /**
     * Get the view finder instance.
     *
     * @return \Illuminate\View\ViewFinderInterface
     */
    public function getFinder()
    {}

    /**
     * Set the view finder instance.
     *
     * @param  \Illuminate\View\ViewFinderInterface  $finder
     * @return void
     */
    public function setFinder(ViewFinderInterface $finder)
    {}

    /**
     * Flush the cache of views located by the finder.
     *
     * @return void
     */
    public function flushFinderCache()
    {}

    /**
     * Get the event dispatcher instance.
     *
     * @return \Illuminate\Contracts\Events\Dispatcher
     */
    public function getDispatcher()
    {}

    /**
     * Set the event dispatcher instance.
     *
     * @param  \Illuminate\Contracts\Events\Dispatcher  $events
     * @return void
     */
    public function setDispatcher(Dispatcher $events)
    {}

    /**
     * Get the IoC container instance.
     *
     * @return \Illuminate\Contracts\Container\Container
     */
    public function getContainer()
    {}

    /**
     * Set the IoC container instance.
     *
     * @param  \Illuminate\Contracts\Container\Container  $container
     * @return void
     */
    public function setContainer(Container $container)
    {}

    /**
     * Get an item from the shared data.
     *
     * @param  string  $key
     * @param  mixed  $default
     * @return mixed
     */
    public function shared($key, $default = null)
    {}

    /**
     * Get all of the shared data for the environment.
     *
     * @return array
     */
    public function getShared()
    {}
}
