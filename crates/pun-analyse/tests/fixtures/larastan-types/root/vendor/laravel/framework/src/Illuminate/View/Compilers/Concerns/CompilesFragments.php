<?php

namespace Illuminate\View\Compilers\Concerns;

trait CompilesFragments
{
    /**
     * The last compiled fragment.
     *
     * @var string
     */
    protected $lastFragment;

    /**
     * Compile the fragment statements into valid PHP.
     *
     * @param  string  $expression
     * @return string
     */
    protected function compileFragment($expression)
    {}

    /**
     * Compile the end-fragment statements into valid PHP.
     *
     * @return string
     */
    protected function compileEndfragment()
    {}
}
