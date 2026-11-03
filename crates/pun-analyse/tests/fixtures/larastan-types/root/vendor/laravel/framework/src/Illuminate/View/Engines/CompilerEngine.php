<?php

namespace Illuminate\View\Engines;

use Illuminate\Database\RecordNotFoundException;
use Illuminate\Database\RecordsNotFoundException;
use Illuminate\Filesystem\Filesystem;
use Illuminate\Http\Exceptions\HttpResponseException;
use Illuminate\Support\Stringable;
use Illuminate\View\Compilers\CompilerInterface;
use Illuminate\View\ViewException;
use Symfony\Component\HttpKernel\Exception\HttpException;
use Throwable;

class CompilerEngine extends PhpEngine
{
    /**
     * The Blade compiler instance.
     *
     * @var \Illuminate\View\Compilers\CompilerInterface
     */
    protected $compiler;

    /**
     * A stack of the last compiled templates.
     *
     * @var array
     */
    protected $lastCompiled = [];

    /**
     * The view paths that were compiled or are not expired, keyed by the path.
     *
     * @var array<string, true>
     */
    protected $compiledOrNotExpired = [];

    /**
     * Create a new compiler engine instance.
     *
     * @param  \Illuminate\View\Compilers\CompilerInterface  $compiler
     * @param  \Illuminate\Filesystem\Filesystem|null  $files
     */
    public function __construct(CompilerInterface $compiler, ?Filesystem $files = null)
    {}

    /**
     * Get the evaluated contents of the view.
     *
     * @param  string  $path
     * @param  array  $data
     * @return string
     *
     * @throws \Illuminate\View\ViewException
     */
    public function get($path, array $data = [])
    {}

    /**
     * Handle a view exception.
     *
     * @param  \Throwable  $e
     * @param  int  $obLevel
     * @return void
     *
     * @throws \Throwable
     */
    protected function handleViewException(Throwable $e, $obLevel)
    {}

    /**
     * Get the exception message for an exception.
     *
     * @param  \Throwable  $e
     * @return string
     */
    protected function getMessage(Throwable $e)
    {}

    /**
     * Get the compiler implementation.
     *
     * @return \Illuminate\View\Compilers\CompilerInterface
     */
    public function getCompiler()
    {}

    /**
     * Clear the cache of views that were compiled or not expired.
     *
     * @return void
     */
    public function forgetCompiledOrNotExpired()
    {}
}
