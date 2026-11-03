<?php

namespace Illuminate\View\Engines;

use Illuminate\Contracts\View\Engine;
use Illuminate\Filesystem\Filesystem;
use Throwable;

class PhpEngine implements Engine
{
    /**
     * The filesystem instance.
     *
     * @var \Illuminate\Filesystem\Filesystem
     */
    protected $files;

    /**
     * Create a new file engine instance.
     *
     * @param  \Illuminate\Filesystem\Filesystem  $files
     */
    public function __construct(Filesystem $files)
    {}

    /**
     * Get the evaluated contents of the view.
     *
     * @param  string  $path
     * @param  array  $data
     * @return string
     */
    public function get($path, array $data = [])
    {}

    /**
     * Get the evaluated contents of the view at the given path.
     *
     * @param  string  $path
     * @param  array  $data
     * @return string
     */
    protected function evaluatePath($path, $data)
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
}
