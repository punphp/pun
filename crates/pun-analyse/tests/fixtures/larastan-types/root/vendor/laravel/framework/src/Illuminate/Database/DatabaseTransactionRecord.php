<?php

namespace Illuminate\Database;

class DatabaseTransactionRecord
{
    /**
     * The name of the database connection.
     *
     * @var string
     */
    public $connection;

    /**
     * The transaction level.
     *
     * @var int
     */
    public $level;

    /**
     * The parent instance of this transaction.
     *
     * @var \Illuminate\Database\DatabaseTransactionRecord|null
     */
    public $parent;

    /**
     * The callbacks that should be executed after committing.
     *
     * @var array
     */
    protected $callbacks = [];

    /**
     * The callbacks that should be executed after rollback.
     *
     * @var array
     */
    protected $callbacksForRollback = [];

    /**
     * Create a new database transaction record instance.
     *
     * @param  string  $connection
     * @param  int  $level
     * @param  \Illuminate\Database\DatabaseTransactionRecord|null  $parent
     */
    public function __construct($connection, $level, ?DatabaseTransactionRecord $parent = null)
    {}

    /**
     * Register a callback to be executed after committing.
     *
     * @param  callable  $callback
     * @return void
     */
    public function addCallback($callback)
    {}

    /**
     * Register a callback to be executed after rollback.
     *
     * @param  callable  $callback
     * @return void
     */
    public function addCallbackForRollback($callback)
    {}

    /**
     * Execute all of the callbacks.
     *
     * @return void
     */
    public function executeCallbacks()
    {}

    /**
     * Execute all of the callbacks for rollback.
     *
     * @return void
     */
    public function executeCallbacksForRollback()
    {}

    /**
     * Get all of the callbacks.
     *
     * @return array
     */
    public function getCallbacks()
    {}

    /**
     * Get all of the callbacks for rollback.
     *
     * @return array
     */
    public function getCallbacksForRollback()
    {}
}
