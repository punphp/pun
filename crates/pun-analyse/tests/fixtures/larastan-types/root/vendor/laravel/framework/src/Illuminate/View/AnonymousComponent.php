<?php

namespace Illuminate\View;

class AnonymousComponent extends Component
{
    /**
     * The component view.
     *
     * @var string
     */
    protected $view;

    /**
     * The component data.
     *
     * @var array
     */
    protected $data = [];

    /**
     * Create a new anonymous component instance.
     *
     * @param  string  $view
     * @param  array  $data
     */
    public function __construct($view, $data)
    {}

    /**
     * Get the view / view contents that represent the component.
     *
     * @return string
     */
    public function render()
    {}

    /**
     * Get the data that should be supplied to the view.
     *
     * @return array
     */
    public function data()
    {}
}
