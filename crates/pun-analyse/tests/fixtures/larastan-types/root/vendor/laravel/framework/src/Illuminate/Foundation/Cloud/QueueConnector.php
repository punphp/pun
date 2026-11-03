<?php

namespace Illuminate\Foundation\Cloud;

use Aws\CommandInterface;
use Aws\Exception\AwsException;
use Aws\Sqs\SqsClient;
use Illuminate\Contracts\Database\LostConnectionDetector;
use Illuminate\Foundation\Application;
use Illuminate\Queue\Connectors\ConnectorInterface;
use Illuminate\Queue\Events\JobQueued;
use Illuminate\Queue\Events\WorkerStopping;
use Illuminate\Queue\SqsQueue;
use Illuminate\Queue\Worker;
use Illuminate\Queue\WorkerStopReason;
use Psr\Http\Message\RequestInterface;

class QueueConnector implements ConnectorInterface
{
    /**
     * Reserved memory so that errors can emit events correctly on memory exhaustion.
     */
    private static ?string $reservedMemory = null;

    /**
     * Create a new instance.
     */
    public function __construct(
        protected ConnectorInterface $connector,
        protected Application $app,
    ) {}

    /**
     * Establish a queue connection.
     */
    public function connect(array $config): Queue
    {}

    /**
     * Register SQS client middleware that translates "queue does not exist" errors into ManagedQueueNotFoundExceptions.
     */
    protected function registerErrorHandling(SqsClient $sqs, Queue $queue): void
    {}

    /**
     * Configure the queue.
     */
    protected function configureQueue(Queue $queue): void
    {}

    /**
     * Configure the queue worker.
     */
    protected function configureWorker(Queue $queue): void
    {}

    /**
     * Configure the failed job provider.
     */
    protected function configureFailedJobProvider(Queue $queue): void
    {}
}
