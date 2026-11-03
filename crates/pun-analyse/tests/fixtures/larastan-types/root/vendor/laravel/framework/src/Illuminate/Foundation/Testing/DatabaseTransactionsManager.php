<?php

namespace Illuminate\Foundation\Testing;

use Illuminate\Database\DatabaseTransactionsManager as BaseManager;

class DatabaseTransactionsManager extends BaseManager
{
    /**
     * The names of the connections transacting during tests.
     */
    protected array $connectionsTransacting;

    /**
     * Create a new database transaction manager instance.
     *
     * @param  array  $connectionsTransacting
     */
    public function __construct(array $connectionsTransacting)
    {}

    /**
     * Register a transaction callback.
     *
     * @param  callable  $callback
     * @return void
     */
    public function addCallback($callback)
    {}

    /**
     * Get the transactions that are applicable to callbacks.
     *
     * @return \Illuminate\Support\Collection<int, \Illuminate\Database\DatabaseTransactionRecord>
     */
    public function callbackApplicableTransactions()
    {}

    /**
     * Determine if after commit callbacks should be executed for the given transaction level.
     *
     * @param  int  $level
     * @return bool
     */
    public function afterCommitCallbacksShouldBeExecuted($level)
    {}
}
