<?php

namespace Illuminate\Foundation;

use Illuminate\Auth\Events\Logout;
use Illuminate\Console\Events\CommandStarting;
use Illuminate\Console\Events\ScheduledTaskFinished;
use Illuminate\Console\Events\ScheduledTaskSkipped;
use Illuminate\Console\Events\ScheduledTaskStarting;
use Illuminate\Contracts\Debug\ExceptionHandler as ExceptionHandlerContract;
use Illuminate\Database\Migrations\Migrator;
use Illuminate\Foundation\Bootstrap\BootProviders;
use Illuminate\Foundation\Bootstrap\HandleExceptions;
use Illuminate\Foundation\Bootstrap\LoadConfiguration;
use Illuminate\Foundation\Cloud\Events;
use Illuminate\Foundation\Cloud\ExceptionReporter;
use Illuminate\Foundation\Cloud\FailedJobProvider;
use Illuminate\Foundation\Cloud\JsonFormatter;
use Illuminate\Foundation\Cloud\QueueConnector;
use Illuminate\Foundation\Exceptions\Renderer\Mappers\BladeMapper;
use Illuminate\Log\Context\Events\ContextDehydrating;
use Illuminate\Queue\Connectors\SqsConnector;
use Illuminate\Queue\Events\JobPopped;
use Illuminate\Queue\Events\Looping;
use Illuminate\Queue\Events\WorkerStopping;
use Illuminate\Support\Arr;
use Illuminate\Support\ConfigurationUrlParser;
use Illuminate\Support\Env;
use Illuminate\Support\Str;
use Laravel\Octane\Events\RequestReceived;
use Monolog\Handler\SocketHandler;
use PDO;
use Throwable;

class CloudBootstrapper
{
    /**
     * Handle a bootstrapper that is bootstrapping.
     */
    public static function bootstrapping(Application $app, string $bootstrapper): void
    {}

    /**
     * Handle a bootstrapper that has bootstrapped.
     */
    public static function bootstrapped(Application $app, string $bootstrapper): void
    {}

    /**
     * Configure the Laravel Cloud disks if applicable.
     */
    public static function configureDisks(Application $app): void
    {}

    /**
     * Configure the Laravel Cloud read replica if applicable.
     */
    public static function configureReadReplicaConnection(Application $app): void
    {}

    /**
     * Configure Laravel Postgres connections, opting into native pooling with DB_POOLING.
     */
    public static function configurePostgresConnections(Application $app): void
    {}

    /**
     * Configure the unpooled Laravel Postgres connection if applicable.
     */
    public static function configureUnpooledPostgresConnection(Application $app): void
    {}

    /**
     * Ensure that migrations use the unpooled Postgres connection if applicable.
     */
    public static function ensureMigrationsUseUnpooledConnection(Application $app): void
    {}

    /**
     * Configure managed queues if applicable.
     *
     * @throws \JsonException
     */
    public static function configureManagedQueues(Application $app): void
    {}

    /**
     * Share cached AWS credentials across processes for all SQS queue connections.
     *
     * Avoids Pod Identity Agent rate limiting.
     */
    public static function configureQueueCredentialCaching(Application $app): void
    {}

    /**
     * Boot managed queues if applicable.
     */
    public static function bootManagedQueues(Application $app): void
    {}

    /**
     * Configure the Laravel Cloud log channels.
     */
    public static function configureCloudLogging(Application $app): void
    {}

    /**
     * Configure the Laravel Cloud exception reporting if applicable.
     *
     * @throws \JsonException
     */
    public static function configureExceptions(Application $app): void
    {}

    /**
     * Register the Laravel Cloud exception reporter if applicable.
     */
    public static function registerExceptionReporting(Application $app): void
    {}

    /**
     * Register the events system for Laravel Cloud.
     */
    public static function registerEvents(Application $app): void
    {}

    /**
     * The cloud socket address.
     */
    protected static function socket(): string
    {}
}
