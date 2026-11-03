<?php

namespace Illuminate\Bus;

use Aws\DynamoDb\DynamoDbClient;
use Aws\DynamoDb\Marshaler;
use Carbon\CarbonImmutable;
use Closure;
use Illuminate\Support\Str;

class DynamoBatchRepository implements BatchRepository
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
     * @var \Aws\DynamoDb\DynamoDbClient
     */
    protected $dynamoDbClient;

    /**
     * The application name.
     *
     * @var string
     */
    protected $applicationName;

    /**
     * The table to use to store batch information.
     *
     * @var string
     */
    protected $table;

    /**
     * The time-to-live value for batch records.
     *
     * @var int|null
     */
    protected $ttl;

    /**
     * The name of the time-to-live attribute for batch records.
     *
     * @var string|null
     */
    protected $ttlAttribute;

    /**
     * The DynamoDB marshaler instance.
     *
     * @var \Aws\DynamoDb\Marshaler
     */
    protected $marshaler;

    /**
     * Create a new batch repository instance.
     */
    public function __construct(
        BatchFactory $factory,
        DynamoDbClient $dynamoDbClient,
        string $applicationName,
        string $table,
        ?int $ttl,
        ?string $ttlAttribute,
    ) {}

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
     * Convert the given raw batch to a Batch object.
     *
     * @param  object  $batch
     * @return \Illuminate\Bus\Batch
     */
    protected function toBatch($batch)
    {}

    /**
     * Create the underlying DynamoDB table.
     *
     * @return void
     */
    public function createAwsDynamoTable(): void
    {}

    /**
     * Delete the underlying DynamoDB table.
     */
    public function deleteAwsDynamoTable(): void
    {}

    /**
     * Get the expiry time based on the configured time-to-live.
     *
     * @return string|null
     */
    protected function getExpiryTime(): ?string
    {}

    /**
     * Get the expression attribute name for the time-to-live attribute.
     *
     * @return array
     */
    protected function ttlExpressionAttributeName(): array
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
     * Get the underlying DynamoDB client instance.
     *
     * @return \Aws\DynamoDb\DynamoDbClient
     */
    public function getDynamoClient(): DynamoDbClient
    {}

    /**
     * The name of the table that contains the batch records.
     *
     * @return string
     */
    public function getTable(): string
    {}
}
