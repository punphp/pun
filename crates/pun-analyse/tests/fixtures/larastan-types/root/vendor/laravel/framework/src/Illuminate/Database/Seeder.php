<?php

namespace Illuminate\Database;

use Illuminate\Console\Command;
use Illuminate\Console\View\Components\TwoColumnDetail;
use Illuminate\Contracts\Container\Container;
use Illuminate\Database\Console\Seeds\WithoutModelEvents;
use Illuminate\Support\Arr;
use InvalidArgumentException;

abstract class Seeder
{
    /**
     * The container instance.
     *
     * @var \Illuminate\Contracts\Container\Container|null
     */
    protected $container;

    /**
     * The console command instance.
     *
     * @var \Illuminate\Console\Command|null
     */
    protected $command;

    /**
     * Seeders that have been called at least one time.
     *
     * @var array
     */
    protected static $called = [];

    /**
     * Run the given seeder class.
     *
     * @param  array|string  $class
     * @param  bool  $silent
     * @param  array  $parameters
     * @return $this
     */
    public function call($class, $silent = false, array $parameters = [])
    {}

    /**
     * Run the given seeder class.
     *
     * @param  array|string  $class
     * @param  array  $parameters
     * @return void
     */
    public function callWith($class, array $parameters = [])
    {}

    /**
     * Silently run the given seeder class.
     *
     * @param  array|string  $class
     * @param  array  $parameters
     * @return void
     */
    public function callSilent($class, array $parameters = [])
    {}

    /**
     * Run the given seeder class once.
     *
     * @param  array|string  $class
     * @param  bool  $silent
     * @return void
     */
    public function callOnce($class, $silent = false, array $parameters = [])
    {}

    /**
     * Resolve an instance of the given seeder class.
     *
     * @param  string  $class
     * @return \Illuminate\Database\Seeder
     */
    protected function resolve($class)
    {}

    /**
     * Set the IoC container instance.
     *
     * @param  \Illuminate\Contracts\Container\Container  $container
     * @return $this
     */
    public function setContainer(Container $container)
    {}

    /**
     * Set the console command instance.
     *
     * @param  \Illuminate\Console\Command  $command
     * @return $this
     */
    public function setCommand(Command $command)
    {}

    /**
     * Run the database seeds.
     *
     * @param  array  $parameters
     * @return mixed
     *
     * @throws \InvalidArgumentException
     */
    public function __invoke(array $parameters = [])
    {}
}
