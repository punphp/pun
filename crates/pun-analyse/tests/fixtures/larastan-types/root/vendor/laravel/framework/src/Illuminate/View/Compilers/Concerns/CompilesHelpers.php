<?php

namespace Illuminate\View\Compilers\Concerns;

use Illuminate\Foundation\Vite;

trait CompilesHelpers
{
    /**
     * Compile the CSRF statements into valid PHP.
     *
     * @return string
     */
    protected function compileCsrf()
    {}

    /**
     * Compile the "dd" statements into valid PHP.
     *
     * @param  string  $arguments
     * @return string
     */
    protected function compileDd($arguments)
    {}

    /**
     * Compile the "dump" statements into valid PHP.
     *
     * @param  string  $arguments
     * @return string
     */
    protected function compileDump($arguments)
    {}

    /**
     * Compile the method statements into valid PHP.
     *
     * @param  string  $method
     * @return string
     */
    protected function compileMethod($method)
    {}

    /**
     * Compile the "vite" statements into valid PHP.
     *
     * @param  string|null  $arguments
     * @return string
     */
    protected function compileVite($arguments)
    {}

    /**
     * Compile the "viteReactRefresh" statements into valid PHP.
     *
     * @return string
     */
    protected function compileViteReactRefresh()
    {}

    /**
     * Compile the "fonts" statements into valid PHP.
     *
     * @param  string|null  $arguments
     * @return string
     */
    protected function compileFonts($arguments)
    {}
}
