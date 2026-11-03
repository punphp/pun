<?php

namespace Illuminate\Database\Migrations;

use Closure;
use Illuminate\Filesystem\Filesystem;
use Illuminate\Support\Facades\Date;
use Illuminate\Support\Str;
use InvalidArgumentException;

class MigrationCreator
{
    /**
     * The filesystem instance.
     *
     * @var \Illuminate\Filesystem\Filesystem
     */
    protected $files;

    /**
     * The custom app stubs directory.
     *
     * @var string
     */
    protected $customStubPath;

    /**
     * The registered post create hooks.
     *
     * @var (\Closure(string, string): void)[]
     */
    protected $postCreate = [];

    /**
     * The path given to the create method.
     *
     * @var string|null
     */
    protected $currentMigrationPath = null;

    /**
     * Create a new migration creator instance.
     *
     * @param  \Illuminate\Filesystem\Filesystem  $files
     * @param  string  $customStubPath
     */
    public function __construct(Filesystem $files, $customStubPath)
    {}

    /**
     * Create a new migration at the given path.
     *
     * @param  string  $name
     * @param  string  $path
     * @param  string|null  $table
     * @param  bool  $create
     * @return string
     *
     * @throws \Exception
     */
    public function create($name, $path, $table = null, $create = false)
    {}

    /**
     * Ensure that a migration with the given name doesn't already exist.
     *
     * @param  string  $name
     * @param  string|null  $migrationPath
     * @return void
     *
     * @throws \InvalidArgumentException
     */
    protected function ensureMigrationDoesntAlreadyExist($name, $migrationPath = null)
    {}

    /**
     * Get the migration stub file.
     *
     * @param  string|null  $table
     * @param  bool  $create
     * @return string
     */
    protected function getStub($table, $create)
    {}

    /**
     * Populate the place-holders in the migration stub.
     *
     * @param  string  $stub
     * @param  string|null  $table
     * @return string
     */
    protected function populateStub($stub, $table)
    {}

    /**
     * Get the class name of a migration name.
     *
     * @param  string  $name
     * @return class-string<\Illuminate\Database\Migrations\Migration>
     */
    protected function getClassName($name)
    {}

    /**
     * Get the full path to the migration.
     *
     * @param  string  $name
     * @param  string  $path
     * @return string
     */
    protected function getPath($name, $path)
    {}

    /**
     * Get the full path to the migration.
     *
     * @param  string  $name
     * @param  string  $path
     * @return string
     */
    protected function getCollisionFreePath($name, $path)
    {}

    /**
     * Fire the registered post create hooks.
     *
     * @param  string|null  $table
     * @param  string  $path
     * @return void
     */
    protected function firePostCreateHooks($table, $path)
    {}

    /**
     * Register a post migration create hook.
     *
     * @param  (\Closure(string, string): void)  $callback
     * @return void
     */
    public function afterCreate(Closure $callback)
    {}

    /**
     * Get the date prefix for the migration.
     *
     * @return string
     */
    protected function getDatePrefix()
    {}

    /**
     * Get the path to the stubs.
     *
     * @return string
     */
    public function stubPath()
    {}

    /**
     * Get the filesystem instance.
     *
     * @return \Illuminate\Filesystem\Filesystem
     */
    public function getFilesystem()
    {}
}
