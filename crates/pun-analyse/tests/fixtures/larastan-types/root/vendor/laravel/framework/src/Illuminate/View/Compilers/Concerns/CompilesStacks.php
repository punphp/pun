<?php

namespace Illuminate\View\Compilers\Concerns;

use Illuminate\Support\Str;

trait CompilesStacks
{
    /**
     * Compile the stack statements into the content.
     *
     * @param  string  $expression
     * @return string
     */
    protected function compileStack($expression)
    {}

    /**
     * Compile the push statements into valid PHP.
     *
     * @param  string  $expression
     * @return string
     */
    protected function compilePush($expression)
    {}

    /**
     * Compile the push-once statements into valid PHP.
     *
     * @param  string  $expression
     * @return string
     */
    protected function compilePushOnce($expression)
    {}

    /**
     * Compile the end-push statements into valid PHP.
     *
     * @return string
     */
    protected function compileEndpush()
    {}

    /**
     * Compile the end-push-once statements into valid PHP.
     *
     * @return string
     */
    protected function compileEndpushOnce()
    {}

    /**
     * Compile the prepend statements into valid PHP.
     *
     * @param  string  $expression
     * @return string
     */
    protected function compilePrepend($expression)
    {}

    /**
     * Compile the prepend-once statements into valid PHP.
     *
     * @param  string  $expression
     * @return string
     */
    protected function compilePrependOnce($expression)
    {}

    /**
     * Compile the end-prepend statements into valid PHP.
     *
     * @return string
     */
    protected function compileEndprepend()
    {}

    /**
     * Compile the end-prepend-once statements into valid PHP.
     *
     * @return string
     */
    protected function compileEndprependOnce()
    {}
}
