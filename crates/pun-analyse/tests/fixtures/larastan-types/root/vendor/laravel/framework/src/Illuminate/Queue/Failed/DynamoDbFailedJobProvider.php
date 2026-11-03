<?php

namespace Illuminate\Queue\Failed;

use Aws\DynamoDb\DynamoDbClient;
use DateTimeInterface;
use Exception;
use Illuminate\Support\Carbon;
use Illuminate\Support\Collection;
use Illuminate\Support\Facades\Date;

class DynamoDbFailedJobProvider implements FailedJobProviderInterface
{
    /**
     * The DynamoDB client instance.
     *
     * @var \Aws\DynamoDb\DynamoDbClient
     */
    protected $dynamo;

    /**
     * The application name.
     *
     * @var string
     */
    protected $applicationName;

    /**
     * The table name.
     *
     * @var string
     */
    protected $table;

    /**
     * Create a new DynamoDb failed job provider.
     *
     * @param  \Aws\DynamoDb\DynamoDbClient  $dynamo
     * @param  string  $applicationName
     * @param  string  $table
     */
    public function __construct(DynamoDbClient $dynamo, $applicationName, $table)
    {}

    /**
     * Log a failed job into storage.
     *
     * @param  string  $connection
     * @param  string  $queue
     * @param  string  $payload
     * @param  \Throwable  $exception
     * @return string|int|null
     */
    public function log($connection, $queue, $payload, $exception)
    {}

    /**
     * Get the IDs of all of the failed jobs.
     *
     * @param  string|null  $queue
     * @return list<string|int>
     */
    public function ids($queue = null)
    {}

    /**
     * Get a list of all of the failed jobs.
     *
     * @return list<object>
     */
    public function all()
    {}

    /**
     * Get a single failed job.
     *
     * @param  mixed  $id
     * @return object|null
     */
    public function find($id)
    {}

    /**
     * Delete a single failed job from storage.
     *
     * @param  mixed  $id
     * @return bool
     */
    public function forget($id)
    {}

    /**
     * Flush the failed jobs from storage.
     *
     * @param  int|null  $hours
     * @param  string|null  $queue
     * @return void
     *
     * @throws \Exception
     */
    public function flush($hours = null, $queue = null)
    {}
}
