<?php

namespace Illuminate\Console;

use Illuminate\Support\Arr;
use Illuminate\Support\Traits\Dumpable;
use Illuminate\Support\Traits\InteractsWithData;

class CommandInput
{
    use Dumpable, InteractsWithData;

    /**
     * The command arguments.
     *
     * @var array
     */
    protected $arguments;

    /**
     * The command options.
     *
     * @var array
     */
    protected $options;

    /**
     * Create a new command input container.
     *
     * @param  array  $arguments
     * @param  array  $options
     */
    public function __construct(array $arguments = [], array $options = [])
    {}

    /**
     * Get all of the input for the command.
     *
     * Options take precedence over arguments when keys collide.
     *
     * @param  mixed  $keys
     * @return array
     */
    public function all($keys = null)
    {}

    /**
     * Retrieve data from the instance.
     *
     * @param  string|null  $key
     * @param  mixed  $default
     * @return mixed
     */
    protected function data($key = null, $default = null)
    {}

    /**
     * Get all of the arguments passed to the command.
     *
     * @return array
     */
    public function arguments()
    {}

    /**
     * Get all of the options passed to the command.
     *
     * @return array
     */
    public function options()
    {}

    /**
     * Get the instance as an array.
     *
     * @return array
     */
    public function toArray()
    {}

    /**
     * Dynamically access input data.
     *
     * @param  string  $name
     * @return mixed
     */
    public function __get($name)
    {}

    /**
     * Determine if an input item is set.
     *
     * @param  string  $name
     * @return bool
     */
    public function __isset($name)
    {}
}
