<?php

namespace Illuminate\View\Concerns;

use Illuminate\Contracts\Support\Htmlable;
use Illuminate\Contracts\View\View;
use Illuminate\Support\Arr;
use Illuminate\View\ComponentSlot;

trait ManagesComponents
{
    /**
     * The components being rendered.
     *
     * @var array
     */
    protected $componentStack = [];

    /**
     * The original data passed to the component.
     *
     * @var array
     */
    protected $componentData = [];

    /**
     * The component data for the component that is currently being rendered.
     *
     * @var array
     */
    protected $currentComponentData = [];

    /**
     * The slot contents for the component.
     *
     * @var array
     */
    protected $slots = [];

    /**
     * The names of the slots being rendered.
     *
     * @var array
     */
    protected $slotStack = [];

    /**
     * Start a component rendering process.
     *
     * @param  \Illuminate\Contracts\View\View|\Illuminate\Contracts\Support\Htmlable|\Closure|string  $view
     * @param  array  $data
     * @return void
     */
    public function startComponent($view, array $data = [])
    {}

    /**
     * Get the first view that actually exists from the given list, and start a component.
     *
     * @param  array  $names
     * @param  array  $data
     * @return void
     */
    public function startComponentFirst(array $names, array $data = [])
    {}

    /**
     * Render the current component.
     *
     * @return string
     */
    public function renderComponent()
    {}

    /**
     * Get the data for the given component.
     *
     * @return array
     */
    protected function componentData()
    {}

    /**
     * Get an item from the component data that exists above the current component.
     *
     * @param  string  $key
     * @param  mixed  $default
     * @return mixed
     */
    public function getConsumableComponentData($key, $default = null)
    {}

    /**
     * Start the slot rendering process.
     *
     * @param  string  $name
     * @param  string|null  $content
     * @param  array  $attributes
     * @return void
     */
    public function slot($name, $content = null, $attributes = [])
    {}

    /**
     * Save the slot content for rendering.
     *
     * @return void
     */
    public function endSlot()
    {}

    /**
     * Get the index for the current component.
     *
     * @return int
     */
    protected function currentComponent()
    {}

    /**
     * Flush all of the component state.
     *
     * @return void
     */
    protected function flushComponents()
    {}
}
