<?php

namespace Illuminate\View\Compilers\Concerns;

trait CompilesSessions
{
    /**
     * Compile the session statements into valid PHP.
     *
     * @param  string  $expression
     * @return string
     */
    protected function compileSession($expression)
    {}

    /**
     * Compile the endsession statements into valid PHP.
     *
     * @param  string  $expression
     * @return string
     */
    protected function compileEndsession($expression)
    {}
}
