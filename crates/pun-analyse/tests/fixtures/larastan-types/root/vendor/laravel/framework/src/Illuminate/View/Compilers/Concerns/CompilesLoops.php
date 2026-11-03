<?php

namespace Illuminate\View\Compilers\Concerns;

use Illuminate\Contracts\View\ViewCompilationException;

trait CompilesLoops
{
    /**
     * Counter to keep track of nested forelse statements.
     *
     * @var int
     */
    protected $forElseCounter = 0;

    /**
     * Compile the for-else statements into valid PHP.
     *
     * @param  string|null  $expression
     * @return string
     *
     * @throws \Illuminate\Contracts\View\ViewCompilationException
     */
    protected function compileForelse($expression)
    {}

    /**
     * Compile the for-else-empty and empty statements into valid PHP.
     *
     * @param  string  $expression
     * @return string
     */
    protected function compileEmpty($expression)
    {}

    /**
     * Compile the end-for-else statements into valid PHP.
     *
     * @return string
     */
    protected function compileEndforelse()
    {}

    /**
     * Compile the end-empty statements into valid PHP.
     *
     * @return string
     */
    protected function compileEndEmpty()
    {}

    /**
     * Compile the for statements into valid PHP.
     *
     * @param  string  $expression
     * @return string
     */
    protected function compileFor($expression)
    {}

    /**
     * Compile the for-each statements into valid PHP.
     *
     * @param  string|null  $expression
     * @return string
     *
     * @throws \Illuminate\Contracts\View\ViewCompilationException
     */
    protected function compileForeach($expression)
    {}

    /**
     * Compile the break statements into valid PHP.
     *
     * @param  string  $expression
     * @return string
     */
    protected function compileBreak($expression)
    {}

    /**
     * Compile the continue statements into valid PHP.
     *
     * @param  string  $expression
     * @return string
     */
    protected function compileContinue($expression)
    {}

    /**
     * Compile the end-for statements into valid PHP.
     *
     * @return string
     */
    protected function compileEndfor()
    {}

    /**
     * Compile the end-for-each statements into valid PHP.
     *
     * @return string
     */
    protected function compileEndforeach()
    {}

    /**
     * Compile the while statements into valid PHP.
     *
     * @param  string  $expression
     * @return string
     */
    protected function compileWhile($expression)
    {}

    /**
     * Compile the end-while statements into valid PHP.
     *
     * @return string
     */
    protected function compileEndwhile()
    {}
}
