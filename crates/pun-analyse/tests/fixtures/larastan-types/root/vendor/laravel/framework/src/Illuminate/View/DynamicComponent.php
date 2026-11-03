<?php

namespace Illuminate\View;

use BackedEnum;
use Illuminate\Container\Container;
use Illuminate\Support\Collection;
use Illuminate\Support\Str;
use Illuminate\View\Compilers\ComponentTagCompiler;

use function Illuminate\Support\enum_value;

class DynamicComponent extends Component
{
    /**
     * The name of the component.
     *
     * @var string
     */
    public $component;

    /**
     * The component tag compiler instance.
     *
     * @var \Illuminate\View\Compilers\ComponentTagCompiler
     */
    protected static $compiler;

    /**
     * The cached component classes.
     *
     * @var array
     */
    protected static $componentClasses = [];

    /**
     * Create a new component instance.
     *
     * @param  \BackedEnum|string  $component
     */
    public function __construct(BackedEnum|string $component)
    {}

    /**
     * Get the view / contents that represent the component.
     *
     * @return \Illuminate\Contracts\View\View|string
     */
    public function render()
    {}

    /**
     * Compile the @props directive for the component.
     *
     * @param  array  $bindings
     * @return string
     */
    protected function compileProps(array $bindings)
    {}

    /**
     * Compile the bindings for the component.
     *
     * @param  array  $bindings
     * @return string
     */
    protected function compileBindings(array $bindings)
    {}

    /**
     * Compile the slots for the component.
     *
     * @param  array  $slots
     * @return string
     */
    protected function compileSlots(array $slots)
    {}

    /**
     * Get the class for the current component.
     *
     * @return string
     */
    protected function classForComponent()
    {}

    /**
     * Get the names of the variables that should be bound to the component.
     *
     * @param  string  $class
     * @return array
     */
    protected function bindings(string $class)
    {}

    /**
     * Get an instance of the Blade tag compiler.
     *
     * @return \Illuminate\View\Compilers\ComponentTagCompiler
     */
    protected function compiler()
    {}
}
