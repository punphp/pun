<?php

namespace Illuminate\View\Compilers\Concerns;

trait CompilesIncludes
{
    /**
     * Compile the each statements into valid PHP.
     *
     * @param  string  $expression
     * @return string
     */
    protected function compileEach($expression)
    {}

    /**
     * Compile the include statements into valid PHP.
     *
     * @param  string  $expression
     * @return string
     */
    protected function compileInclude($expression)
    {}

    /**
     * Compile the include-if statements into valid PHP.
     *
     * @param  string  $expression
     * @return string
     */
    protected function compileIncludeIf($expression)
    {}

    /**
     * Compile the include-when statements into valid PHP.
     *
     * @param  string  $expression
     * @return string
     */
    protected function compileIncludeWhen($expression)
    {}

    /**
     * Compile the include-unless statements into valid PHP.
     *
     * @param  string  $expression
     * @return string
     */
    protected function compileIncludeUnless($expression)
    {}

    /**
     * Compile the include-first statements into valid PHP.
     *
     * @param  string  $expression
     * @return string
     */
    protected function compileIncludeFirst($expression)
    {}

    /**
     * Compile the include-isolated statements into valid PHP.
     *
     * @param  string  $expression
     * @return string
     */
    protected function compileIncludeIsolated($expression)
    {}
}
