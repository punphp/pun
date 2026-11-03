<?php

namespace Illuminate\Console\View\Components;

use Illuminate\Console\OutputStyle;
use Illuminate\Console\QuestionHelper;
use ReflectionClass;
use Symfony\Component\Console\Helper\SymfonyQuestionHelper;

use function Termwind\render;
use function Termwind\renderUsing;

abstract class Component
{
    /**
     * The output style implementation.
     *
     * @var \Illuminate\Console\OutputStyle
     */
    protected $output;

    /**
     * The list of mutators to apply on the view data.
     *
     * @var array<int, callable(string): string>
     */
    protected $mutators;

    /**
     * Creates a new component instance.
     *
     * @param  \Illuminate\Console\OutputStyle  $output
     */
    public function __construct($output)
    {}

    /**
     * Renders the given view.
     *
     * @param  string  $view
     * @param  \Illuminate\Contracts\Support\Arrayable|array  $data
     * @param  int  $verbosity
     * @return void
     */
    protected function renderView($view, $data, $verbosity)
    {}

    /**
     * Compile the given view contents.
     *
     * @param  string  $view
     * @param  array  $data
     * @return string
     */
    protected function compile($view, $data)
    {}

    /**
     * Mutates the given data with the given set of mutators.
     *
     * @param  array<int, string>|string  $data
     * @param  array<int, callable(string): string>  $mutators
     * @return array<int, string>|string
     */
    protected function mutate($data, $mutators)
    {}

    /**
     * Eventually performs a question using the component's question helper.
     *
     * @param  callable  $callable
     * @return mixed
     */
    protected function usingQuestionHelper($callable)
    {}
}
