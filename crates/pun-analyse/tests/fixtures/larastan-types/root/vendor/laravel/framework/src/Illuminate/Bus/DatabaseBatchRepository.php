<?php

namespace Illuminate\Bus;

use Carbon\CarbonImmutable;
use Closure;
use DateTimeInterface;
use Illuminate\Database\Connection;
use Illuminate\Database\PostgresConnection;
use Illuminate\Database\Query\Expression;
use Illuminate\Support\Str;
use Throwable;

class DatabaseBatchRepository implements PrunableBatchRepository
{
    /**
     * The batch factory instance.
     *
     * @var \Illuminate\Bus\BatchFactory
     */
    protected $factory;

    /**
     * The database connection instance.
     *
     * @var \Illuminate\Database\Connection
     */
    protected $connection;

    /**
     * The database table to use to store batch information.
     *
     * @var string
     */
    protected $table;

    /**
     * Create a new batch repository instance.
     *
     * @param  \Illuminate\Bus\BatchFactory  $factory
     * @param  \Illuminate\Database\Connection  $connection
     * @param  string  $table
     */
    public function __construct(BatchFactory $factory, Connection $connection, string $table)
    {}

    /**
     * Retrieve a list of batches.
     *
     * @param  int  $limit
     * @param  mixed  $before
     * @return \Illuminate\Bus\Batch[]
     */
    public function get($limit = 50, $before = null)
    {}

    /**
     * Retrieve information about an existing batch.
     *
     * @param  string  $batchId
     * @return \Illuminate\Bus\Batch|null
     */
    public function find(string $batchId)
    {}

    /**
     * Store a new pending batch.
     *
     * @param  \Illuminate\Bus\PendingBatch  $batch
     * @return \Illuminate\Bus\Batch
     */
    public function store(PendingBatch $batch)
    {}

    /**
     * Increment the total number of jobs within the batch.
     *
     * @param  string  $batchId
     * @param  int  $amount
     * @return void
     */
    public function incrementTotalJobs(string $batchId, int $amount)
    {}

    /**
     * Decrement the total number of pending jobs for the batch.
     *
     * @param  string  $batchId
     * @param  string  $jobId
     * @return \Illuminate\Bus\UpdatedBatchJobCounts
     */
    public function decrementPendingJobs(string $batchId, string $jobId)
    {}

    /**
     * Increment the total number of failed jobs for the batch.
     *
     * @param  string  $batchId
     * @param  string  $jobId
     * @return \Illuminate\Bus\UpdatedBatchJobCounts
     */
    public function incrementFailedJobs(string $batchId, string $jobId)
    {}

    /**
     * Update an atomic value within the batch.
     *
     * @param  string  $batchId
     * @param  \Closure  $callback
     * @return int|null
     */
    protected function updateAtomicValues(string $batchId, Closure $callback)
    {}

    /**
     * Mark the batch that has the given ID as finished.
     *
     * @param  string  $batchId
     * @return void
     */
    public function markAsFinished(string $batchId)
    {}

    /**
     * Cancel the batch that has the given ID.
     *
     * @param  string  $batchId
     * @return void
     */
    public function cancel(string $batchId)
    {}

    /**
     * Delete the batch that has the given ID.
     *
     * @param  string  $batchId
     * @return void
     */
    public function delete(string $batchId)
    {}

    /**
     * Prune all of the entries older than the given date.
     *
     * @param  \DateTimeInterface  $before
     * @return int
     */
    public function prune(DateTimeInterface $before)
    {}

    /**
     * Prune all of the unfinished entries older than the given date.
     *
     * @param  \DateTimeInterface  $before
     * @return int
     */
    public function pruneUnfinished(DateTimeInterface $before)
    {}

    /**
     * Prune all of the cancelled entries older than the given date.
     *
     * @param  \DateTimeInterface  $before
     * @return int
     */
    public function pruneCancelled(DateTimeInterface $before)
    {}

    /**
     * Execute the given Closure within a storage specific transaction.
     *
     * @template TReturn
     *
     * @param  (\Closure(): TReturn)  $callback
     * @return TReturn
     */
    public function transaction(Closure $callback)
    {}

    /**
     * Rollback the last database transaction for the connection.
     *
     * @return void
     */
    public function rollBack()
    {}

    /**
     * Serialize the given value.
     *
     * @param  mixed  $value
     * @return string
     */
    protected function serialize($value)
    {}

    /**
     * Unserialize the given value.
     *
     * @param  string  $serialized
     * @return mixed
     */
    protected function unserialize($serialized)
    {}

    /**
     * Convert the given raw batch to a Batch object.
     *
     * @param  object  $batch
     * @return \Illuminate\Bus\Batch
     */
    protected function toBatch($batch)
    {}

    /**
     * Get the underlying database connection.
     *
     * @return \Illuminate\Database\Connection
     */
    public function getConnection()
    {}

    /**
     * Set the underlying database connection.
     *
     * @param  \Illuminate\Database\Connection  $connection
     * @return void
     */
    public function setConnection(Connection $connection)
    {}
}
