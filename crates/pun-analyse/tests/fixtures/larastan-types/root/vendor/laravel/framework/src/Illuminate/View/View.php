<?php

namespace Illuminate\View;

use ArrayAccess;
use BadMethodCallException;
use Illuminate\Contracts\Support\Arrayable;
use Illuminate\Contracts\Support\Htmlable;
use Illuminate\Contracts\Support\MessageProvider;
use Illuminate\Contracts\Support\Renderable;
use Illuminate\Contracts\View\Engine;
use Illuminate\Contracts\View\View as ViewContract;
use Illuminate\Support\Collection;
use Illuminate\Support\MessageBag;
use Illuminate\Support\Str;
use Illuminate\Support\Traits\Macroable;
use Illuminate\Support\ViewErrorBag;
use Stringable;
use Throwable;

class View implements ArrayAccess, Htmlable, Stringable, ViewContract
{
    use Macroable {
        __call as macroCall;
    }

    /**
     * The view factory instance.
     *
     * @var \Illuminate\View\Factory
     */
    protected $factory;

    /**
     * The engine implementation.
     *
     * @var \Illuminate\Contracts\View\Engine
     */
    protected $engine;

    /**
     * The name of the view.
     *
     * @var string
     */
    protected $view;

    /**
     * The array of view data.
     *
     * @var array
     */
    protected $data;

    /**
     * The path to the view file.
     *
     * @var string
     */
    protected $path;

    /**
     * Create a new view instance.
     *
     * @param  \Illuminate\View\Factory  $factory
     * @param  \Illuminate\Contracts\View\Engine  $engine
     * @param  string  $view
     * @param  string  $path
     * @param  mixed  $data
     */
    public function __construct(Factory $factory, Engine $engine, $view, $path, $data = [])
    {}

    /**
     * Get the evaluated contents of a given fragment.
     *
     * @param  string  $fragment
     * @return string
     */
    public function fragment($fragment)
    {}

    /**
     * Get the evaluated contents for a given array of fragments or return all fragments.
     *
     * @param  array|null  $fragments
     * @return string
     */
    public function fragments(?array $fragments = null)
    {}

    /**
     * Get the evaluated contents of a given fragment if the given condition is true.
     *
     * @param  bool  $boolean
     * @param  string  $fragment
     * @return string
     */
    public function fragmentIf($boolean, $fragment)
    {}

    /**
     * Get the evaluated contents for a given array of fragments if the given condition is true.
     *
     * @param  bool  $boolean
     * @param  array|null  $fragments
     * @return string
     */
    public function fragmentsIf($boolean, ?array $fragments = null)
    {}

    /**
     * Get all fragments as a single string.
     *
     * @return string
     */
    protected function allFragments()
    {}

    /**
     * Get the string contents of the view.
     *
     * @param  callable|null  $callback
     * @return string
     *
     * @throws \Throwable
     */
    public function render(?callable $callback = null)
    {}

    /**
     * Get the contents of the view instance.
     *
     * @return string
     */
    protected function renderContents()
    {}

    /**
     * Get the evaluated contents of the view.
     *
     * @return string
     */
    protected function getContents()
    {}

    /**
     * Get the data bound to the view instance.
     *
     * @return array
     */
    public function gatherData()
    {}

    /**
     * Get the sections of the rendered view.
     *
     * @return array
     *
     * @throws \Throwable
     */
    public function renderSections()
    {}

    /**
     * Add a piece of data to the view.
     *
     * @param  string|array  $key
     * @param  mixed  $value
     * @return $this
     */
    public function with($key, $value = null)
    {}

    /**
     * Add a view instance to the view data.
     *
     * @param  string  $key
     * @param  string  $view
     * @param  array  $data
     * @return $this
     */
    public function nest($key, $view, array $data = [])
    {}

    /**
     * Add validation errors to the view.
     *
     * @param  \Illuminate\Contracts\Support\MessageProvider|array|string  $provider
     * @param  string  $bag
     * @return $this
     */
    public function withErrors($provider, $bag = 'default')
    {}

    /**
     * Parse the given errors into an appropriate value.
     *
     * @param  \Illuminate\Contracts\Support\MessageProvider|array|string  $provider
     * @return \Illuminate\Support\MessageBag
     */
    protected function formatErrors($provider)
    {}

    /**
     * Get the name of the view.
     *
     * @return string
     */
    public function name()
    {}

    /**
     * Get the name of the view.
     *
     * @return string
     */
    public function getName()
    {}

    /**
     * Get the array of view data.
     *
     * @return array
     */
    public function getData()
    {}

    /**
     * Get the path to the view file.
     *
     * @return string
     */
    public function getPath()
    {}

    /**
     * Set the path to the view.
     *
     * @param  string  $path
     * @return void
     */
    public function setPath($path)
    {}

    /**
     * Get the view factory instance.
     *
     * @return \Illuminate\View\Factory
     */
    public function getFactory()
    {}

    /**
     * Get the view's rendering engine.
     *
     * @return \Illuminate\Contracts\View\Engine
     */
    public function getEngine()
    {}

    /**
     * Determine if a piece of data is bound.
     *
     * @param  string  $offset
     * @return bool
     */
    public function offsetExists($offset): bool
    {}

    /**
     * Get a piece of bound data to the view.
     *
     * @param  string  $offset
     * @return mixed
     */
    public function offsetGet($offset): mixed
    {}

    /**
     * Set a piece of data on the view.
     *
     * @param  string  $offset
     * @param  mixed  $value
     * @return void
     */
    public function offsetSet($offset, $value): void
    {}

    /**
     * Unset a piece of data from the view.
     *
     * @param  string  $offset
     * @return void
     */
    public function offsetUnset($offset): void
    {}

    /**
     * Get a piece of data from the view.
     *
     * @param  string  $key
     * @return mixed
     */
    public function &__get($key)
    {}

    /**
     * Set a piece of data on the view.
     *
     * @param  string  $key
     * @param  mixed  $value
     * @return void
     */
    public function __set($key, $value)
    {}

    /**
     * Check if a piece of data is bound to the view.
     *
     * @param  string  $key
     * @return bool
     */
    public function __isset($key)
    {}

    /**
     * Remove a piece of bound data from the view.
     *
     * @param  string  $key
     * @return void
     */
    public function __unset($key)
    {}

    /**
     * Dynamically bind parameters to the view.
     *
     * @param  string  $method
     * @param  array  $parameters
     * @return \Illuminate\View\View
     *
     * @throws \BadMethodCallException
     */
    public function __call($method, $parameters)
    {}

    /**
     * Get content as a string of HTML.
     *
     * @return string
     */
    public function toHtml()
    {}

    /**
     * Get the string contents of the view.
     *
     * @return string
     *
     * @throws \Throwable
     */
    public function __toString()
    {}
}
