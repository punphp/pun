<?php

namespace Illuminate\Database\Concerns;

use Closure;
use Illuminate\Database\DeadlockException;
use RuntimeException;
use Throwable;

/**
 * @mixin \Illuminate\Database\Connection
 */
trait ManagesTransactions
{
    /**
     * @template TReturn of mixed
     *
     * Execute a Closure within a transaction.
     *
     * @param  (\Closure(static): TReturn)  $callback
     * @param  int  $attempts
     * @return TReturn
     *
     * @throws \Throwable
     */
    public function transaction(Closure $callback, $attempts = 1)
    {}

    /**
     * Handle an exception encountered when running a transacted statement.
     *
     * @param  \Throwable  $e
     * @param  int  $currentAttempt
     * @param  int  $maxAttempts
     * @return void
     *
     * @throws \Throwable
     */
    protected function handleTransactionException(Throwable $e, $currentAttempt, $maxAttempts)
    {}

    /**
     * Start a new database transaction.
     *
     * @return void
     *
     * @throws \Throwable
     */
    public function beginTransaction()
    {}

    /**
     * Create a transaction within the database.
     *
     * @return void
     *
     * @throws \Throwable
     */
    protected function createTransaction()
    {}

    /**
     * Create a save point within the database.
     *
     * @return void
     *
     * @throws \Throwable
     */
    protected function createSavepoint()
    {}

    /**
     * Handle an exception from a transaction beginning.
     *
     * @param  \Throwable  $e
     * @return void
     *
     * @throws \Throwable
     */
    protected function handleBeginTransactionException(Throwable $e)
    {}

    /**
     * Commit the active database transaction.
     *
     * @return void
     *
     * @throws \Throwable
     */
    public function commit()
    {}

    /**
     * Handle an exception encountered when committing a transaction.
     *
     * @param  \Throwable  $e
     * @param  int  $currentAttempt
     * @param  int  $maxAttempts
     * @return void
     *
     * @throws \Throwable
     */
    protected function handleCommitTransactionException(Throwable $e, $currentAttempt, $maxAttempts)
    {}

    /**
     * Rollback the active database transaction.
     *
     * @param  int|null  $toLevel
     * @return void
     *
     * @throws \Throwable
     */
    public function rollBack($toLevel = null)
    {}

    /**
     * Perform a rollback within the database.
     *
     * @param  int  $toLevel
     * @return void
     *
     * @throws \Throwable
     */
    protected function performRollBack($toLevel)
    {}

    /**
     * Handle an exception from a rollback.
     *
     * @param  \Throwable  $e
     * @return void
     *
     * @throws \Throwable
     */
    protected function handleRollBackException(Throwable $e)
    {}

    /**
     * Get the number of active transactions.
     *
     * @return int
     */
    public function transactionLevel()
    {}

    /**
     * Execute the callback after a transaction commits.
     *
     * @param  callable  $callback
     * @return void
     *
     * @throws \RuntimeException
     */
    public function afterCommit($callback)
    {}

    /**
     * Execute the callback after a transaction rolls back.
     *
     * @param  callable  $callback
     * @return void
     *
     * @throws \RuntimeException
     */
    public function afterRollBack($callback)
    {}
}
