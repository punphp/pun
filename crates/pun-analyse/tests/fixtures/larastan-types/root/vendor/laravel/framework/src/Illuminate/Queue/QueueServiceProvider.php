<?php

namespace Illuminate\Queue;

use Aws\DynamoDb\DynamoDbClient;
use Illuminate\Contracts\Debug\ExceptionHandler;
use Illuminate\Contracts\Events\Dispatcher as EventDispatcher;
use Illuminate\Contracts\Support\DeferrableProvider;
use Illuminate\Queue\Connectors\BackgroundConnector;
use Illuminate\Queue\Connectors\BeanstalkdConnector;
use Illuminate\Queue\Connectors\DatabaseConnector;
use Illuminate\Queue\Connectors\DeferredConnector;
use Illuminate\Queue\Connectors\FailoverConnector;
use Illuminate\Queue\Connectors\NullConnector;
use Illuminate\Queue\Connectors\RedisConnector;
use Illuminate\Queue\Connectors\SqsConnector;
use Illuminate\Queue\Connectors\SyncConnector;
use Illuminate\Queue\Failed\DatabaseFailedJobProvider;
use Illuminate\Queue\Failed\DatabaseUuidFailedJobProvider;
use Illuminate\Queue\Failed\DynamoDbFailedJobProvider;
use Illuminate\Queue\Failed\FileFailedJobProvider;
use Illuminate\Queue\Failed\NullFailedJobProvider;
use Illuminate\Support\Arr;
use Illuminate\Support\Facades\Facade;
use Illuminate\Support\ServiceProvider;
use Laravel\SerializableClosure\SerializableClosure;

class QueueServiceProvider extends ServiceProvider implements DeferrableProvider
{
    use SerializesAndRestoresModelIdentifiers;

    /**
     * Register the service provider.
     *
     * @return void
     */
    public function register()
    {}

    /**
     * Configure serializable closures uses.
     *
     * @return void
     */
    protected function configureSerializableClosureUses()
    {}

    /**
     * Register the queue manager.
     *
     * @return void
     */
    protected function registerManager()
    {}

    /**
     * Register the default queue connection binding.
     *
     * @return void
     */
    protected function registerConnection()
    {}

    /**
     * Register the connectors on the queue manager.
     *
     * @param  \Illuminate\Queue\QueueManager  $manager
     * @return void
     */
    public function registerConnectors($manager)
    {}

    /**
     * Register the Null queue connector.
     *
     * @param  \Illuminate\Queue\QueueManager  $manager
     * @return void
     */
    protected function registerNullConnector($manager)
    {}

    /**
     * Register the Sync queue connector.
     *
     * @param  \Illuminate\Queue\QueueManager  $manager
     * @return void
     */
    protected function registerSyncConnector($manager)
    {}

    /**
     * Register the Deferred queue connector.
     *
     * @param  \Illuminate\Queue\QueueManager  $manager
     * @return void
     */
    protected function registerDeferredConnector($manager)
    {}

    /**
     * Register the Background queue connector.
     *
     * @param  \Illuminate\Queue\QueueManager  $manager
     * @return void
     */
    protected function registerBackgroundConnector($manager)
    {}

    /**
     * Register the Failover queue connector.
     *
     * @param  \Illuminate\Queue\QueueManager  $manager
     * @return void
     */
    protected function registerFailoverConnector($manager)
    {}

    /**
     * Register the database queue connector.
     *
     * @param  \Illuminate\Queue\QueueManager  $manager
     * @return void
     */
    protected function registerDatabaseConnector($manager)
    {}

    /**
     * Register the Redis queue connector.
     *
     * @param  \Illuminate\Queue\QueueManager  $manager
     * @return void
     */
    protected function registerRedisConnector($manager)
    {}

    /**
     * Register the Beanstalkd queue connector.
     *
     * @param  \Illuminate\Queue\QueueManager  $manager
     * @return void
     */
    protected function registerBeanstalkdConnector($manager)
    {}

    /**
     * Register the Amazon SQS queue connector.
     *
     * @param  \Illuminate\Queue\QueueManager  $manager
     * @return void
     */
    protected function registerSqsConnector($manager)
    {}

    /**
     * Register the queue worker.
     *
     * @return void
     */
    protected function registerWorker()
    {}

    /**
     * Register the queue listener.
     *
     * @return void
     */
    protected function registerListener()
    {}

    /**
     * Register the default queue routes binding.
     *
     * @return void
     */
    protected function registerRoutes()
    {}

    /**
     * Register the failed job services.
     *
     * @return void
     */
    protected function registerFailedJobServices()
    {}

    /**
     * Create a new database failed job provider.
     *
     * @param  array  $config
     * @return \Illuminate\Queue\Failed\DatabaseFailedJobProvider
     */
    protected function databaseFailedJobProvider($config)
    {}

    /**
     * Create a new database failed job provider that uses UUIDs as IDs.
     *
     * @param  array  $config
     * @return \Illuminate\Queue\Failed\DatabaseUuidFailedJobProvider
     */
    protected function databaseUuidFailedJobProvider($config)
    {}

    /**
     * Create a new DynamoDb failed job provider.
     *
     * @param  array  $config
     * @return \Illuminate\Queue\Failed\DynamoDbFailedJobProvider
     */
    protected function dynamoFailedJobProvider($config)
    {}

    /**
     * Get the services provided by the provider.
     *
     * @return array
     */
    public function provides()
    {}
}
