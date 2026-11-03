<?php

namespace Illuminate\Foundation;

class AliasLoader
{
    /**
     * The array of class aliases.
     *
     * @var array
     */
    protected $aliases;

    /**
     * Indicates if a loader has been registered.
     *
     * @var bool
     */
    protected $registered = false;

    /**
     * The namespace for all real-time facades.
     *
     * @var string
     */
    protected static $facadeNamespace = 'Facades\\';

    /**
     * The singleton instance of the loader.
     *
     * @var \Illuminate\Foundation\AliasLoader|null
     */
    protected static $instance;

    /**
     * Create a new AliasLoader instance.
     *
     * @param  array  $aliases
     */
    private function __construct($aliases)
    {}

    /**
     * Get or create the singleton alias loader instance.
     *
     * @param  array  $aliases
     * @return \Illuminate\Foundation\AliasLoader
     */
    public static function getInstance(array $aliases = [])
    {}

    /**
     * Load a class alias if it is registered.
     *
     * @param  string  $alias
     * @return bool|null
     */
    public function load($alias)
    {}

    /**
     * Load a real-time facade for the given alias.
     *
     * @param  string  $alias
     * @return void
     */
    protected function loadFacade($alias)
    {}

    /**
     * Ensure that the given alias has an existing real-time facade class.
     *
     * @param  string  $alias
     * @return string
     */
    protected function ensureFacadeExists($alias)
    {}

    /**
     * Format the facade stub with the proper namespace and class.
     *
     * @param  string  $alias
     * @param  string  $stub
     * @return string
     */
    protected function formatFacadeStub($alias, $stub)
    {}

    /**
     * Add an alias to the loader.
     *
     * @param  string  $alias
     * @param  string  $class
     * @return void
     */
    public function alias($alias, $class)
    {}

    /**
     * Register the loader on the auto-loader stack.
     *
     * @return void
     */
    public function register()
    {}

    /**
     * Prepend the load method to the auto-loader stack.
     *
     * @return void
     */
    protected function prependToLoaderStack()
    {}

    /**
     * Get the registered aliases.
     *
     * @return array
     */
    public function getAliases()
    {}

    /**
     * Set the registered aliases.
     *
     * @param  array  $aliases
     * @return void
     */
    public function setAliases(array $aliases)
    {}

    /**
     * Indicates if the loader has been registered.
     *
     * @return bool
     */
    public function isRegistered()
    {}

    /**
     * Set the "registered" state of the loader.
     *
     * @param  bool  $value
     * @return void
     */
    public function setRegistered($value)
    {}

    /**
     * Set the real-time facade namespace.
     *
     * @param  string  $namespace
     * @return void
     */
    public static function setFacadeNamespace($namespace)
    {}

    /**
     * Set the value of the singleton alias loader.
     *
     * @param  \Illuminate\Foundation\AliasLoader  $loader
     * @return void
     */
    public static function setInstance($loader)
    {}

    /**
     * Clone method.
     *
     * @return void
     */
    private function __clone()
    {}
}
