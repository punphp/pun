<?php

namespace Illuminate\Database;

use InvalidArgumentException;

class SQLiteDatabaseDoesNotExistException extends InvalidArgumentException
{
    /**
     * The path to the database.
     *
     * @var string
     */
    public $path;

    /**
     * Create a new exception instance.
     *
     * @param  string  $path
     */
    public function __construct($path)
    {}
}
