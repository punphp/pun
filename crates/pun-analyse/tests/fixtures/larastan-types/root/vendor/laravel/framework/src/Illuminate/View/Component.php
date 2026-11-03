<?php

namespace Illuminate\View;

use Closure;
use Illuminate\Container\Container;
use Illuminate\Contracts\Support\Htmlable;
use Illuminate\Contracts\View\View as ViewContract;
use Illuminate\Filesystem\Filesystem;
use Illuminate\Support\Collection;
use ReflectionClass;
use ReflectionMethod;
use ReflectionProperty;

abstract class Component
{
    /**
     * The properties / methods that should not be exposed to the component.
     *
     * @var array
     */
    protected $except = [];

    /**
     * The component alias name.
     *
     * @var string
     */
    public $componentName;

    /**
     * The component attributes.
     *
     * @var \Illuminate\View\ComponentAttributeBag
     */
    public $attributes;

    /**
     * The view factory instance, if any.
     *
     * @var \Illuminate\Contracts\View\Factory|null
     */
    protected static $factory;

    /**
     * The component resolver callback.
     *
     * @var (\Closure(string, array): Component)|null
     */
    protected static $componentsResolver;

    /**
     * The cache of blade view names, keyed by contents.
     *
     * @var array<string, string>
     */
    protected static $bladeViewCache = [];

    /**
     * The cache of public property names, keyed by class.
     *
     * @var array
     */
    protected static $propertyCache = [];

    /**
     * The cache of public method names, keyed by class.
     *
     * @var array
     */
    protected static $methodCache = [];

    /**
     * The cache of constructor parameters, keyed by class.
     *
     * @var array<class-string, array<int, string>>
     */
    protected static $constructorParametersCache = [];

    /**
     * The cache of ignored parameter names.
     *
     * @var array
     */
    protected static $ignoredParameterNames = [];

    /**
     * Get the view / view contents that represent the component.
     *
     * @return \Illuminate\Contracts\View\View|\Illuminate\Contracts\Support\Htmlable|\Closure|string
     */
    abstract public function render();

    /**
     * Resolve the component instance with the given data.
     *
     * @param  array  $data
     * @return static
     */
    public static function resolve($data)
    {}

    /**
     * Extract the constructor parameters for the component.
     *
     * @return array
     */
    protected static function extractConstructorParameters()
    {}

    /**
     * Resolve the Blade view or view file that should be used when rendering the component.
     *
     * @return \Illuminate\Contracts\View\View|\Illuminate\Contracts\Support\Htmlable|\Closure|string
     */
    public function resolveView()
    {}

    /**
     * Create a Blade view with the raw component string content.
     *
     * @param  string  $contents
     * @return string
     */
    protected function extractBladeViewFromString($contents)
    {}

    /**
     * Create a Blade view with the raw component string content.
     *
     * @param  \Illuminate\Contracts\View\Factory  $factory
     * @param  string  $contents
     * @return string
     */
    protected function createBladeViewFromString($factory, $contents)
    {}

    /**
     * Get the data that should be supplied to the view.
     *
     * @author Freek Van der Herten
     * @author Brent Roose
     *
     * @return array
     */
    public function data()
    {}

    /**
     * Extract the public properties for the component.
     *
     * @return array
     */
    protected function extractPublicProperties()
    {}

    /**
     * Extract the public methods for the component.
     *
     * @return array
     */
    protected function extractPublicMethods()
    {}

    /**
     * Create a callable variable from the given method.
     *
     * @param  \ReflectionMethod  $method
     * @return mixed
     */
    protected function createVariableFromMethod(ReflectionMethod $method)
    {}

    /**
     * Create an invokable, toStringable variable for the given component method.
     *
     * @param  string  $method
     * @return \Illuminate\View\InvokableComponentVariable<mixed>
     */
    protected function createInvokableVariable(string $method)
    {}

    /**
     * Determine if the given property / method should be ignored.
     *
     * @param  string  $name
     * @return bool
     */
    protected function shouldIgnore($name)
    {}

    /**
     * Get the methods that should be ignored.
     *
     * @return array
     */
    protected function ignoredMethods()
    {}

    /**
     * Set the component alias name.
     *
     * @param  string  $name
     * @return $this
     */
    public function withName($name)
    {}

    /**
     * Set the extra attributes that the component should make available.
     *
     * @param  array  $attributes
     * @return $this
     */
    public function withAttributes(array $attributes)
    {}

    /**
     * Get a new attribute bag instance.
     *
     * @param  array  $attributes
     * @return \Illuminate\View\ComponentAttributeBag
     */
    protected function newAttributeBag(array $attributes = [])
    {}

    /**
     * Determine if the component should be rendered.
     *
     * @return bool
     */
    public function shouldRender()
    {}

    /**
     * Get the evaluated view contents for the given view.
     *
     * @param  string|null  $view
     * @param  \Illuminate\Contracts\Support\Arrayable|array  $data
     * @param  array  $mergeData
     * @return \Illuminate\Contracts\View\View
     */
    public function view($view, $data = [], $mergeData = [])
    {}

    /**
     * Get the view factory instance.
     *
     * @return \Illuminate\Contracts\View\Factory
     */
    protected function factory()
    {}

    /**
     * Get the cached set of anonymous component constructor parameter names to exclude.
     *
     * @return array
     */
    public static function ignoredParameterNames()
    {}

    /**
     * Flush the component's cached state.
     *
     * @return void
     */
    public static function flushCache()
    {}

    /**
     * Forget the component's factory instance.
     *
     * @return void
     */
    public static function forgetFactory()
    {}

    /**
     * Forget the component's resolver callback.
     *
     * @return void
     *
     * @internal
     */
    public static function forgetComponentsResolver()
    {}

    /**
     * Set the callback that should be used to resolve components within views.
     *
     * @param  \Closure(string $component, array $data): Component  $resolver
     * @return void
     *
     * @internal
     */
    public static function resolveComponentsUsing($resolver)
    {}
}
