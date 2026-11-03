<?php

namespace Illuminate\Queue\Connectors;

use Aws\Credentials\CredentialProvider;
use Aws\Sqs\SqsClient;
use Illuminate\Container\Container;
use Illuminate\Queue\AwsCredentialCache;
use Illuminate\Queue\SqsQueue;
use Illuminate\Support\Arr;
use InvalidArgumentException;

class SqsConnector implements ConnectorInterface
{
    /**
     * Establish a queue connection.
     *
     * @param  array  $config
     * @return \Illuminate\Contracts\Queue\Queue
     */
    public function connect(array $config)
    {}

    /**
     * Get the default configuration for SQS.
     *
     * @param  array  $config
     * @return array
     */
    protected function getDefaultConfiguration(array $config)
    {}

    /**
     * Configure the credentials for the given connection config.
     *
     * @param  array  $config
     * @return array
     */
    protected function withCredentials(array $config)
    {}

    /**
     * Resolve a credential provider from the given config.
     *
     * @param  array  $config
     * @return callable|null
     *
     * @throws \InvalidArgumentException
     */
    protected function resolveCredentialProvider(array $config)
    {}

    /**
     * Wrap the given credential provider so the credentials it resolves are shared across processes via the cache.
     *
     * @param  callable  $provider
     * @param  array  $config
     * @return callable
     */
    protected function cachedCredentialProvider(callable $provider, array $config)
    {}

    /**
     * Get the cache key for the connection's shared credentials.
     *
     * @param  array  $config
     * @return string
     */
    public static function credentialsCacheKey(array $config)
    {}

    /**
     * Determine if resolved credentials should be shared across processes via the cache.
     *
     * @param  array  $config
     * @return bool
     */
    protected function credentialCachingEnabled(array $config)
    {}
}
