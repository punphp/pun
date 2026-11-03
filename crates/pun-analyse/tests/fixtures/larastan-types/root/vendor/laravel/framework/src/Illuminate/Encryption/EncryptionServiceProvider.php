<?php

namespace Illuminate\Encryption;

use Illuminate\Support\ServiceProvider;
use Illuminate\Support\Str;
use Laravel\SerializableClosure\SerializableClosure;

class EncryptionServiceProvider extends ServiceProvider
{
    /**
     * Register the service provider.
     *
     * @return void
     */
    public function register()
    {}

    /**
     * Register the encrypter.
     *
     * @return void
     */
    protected function registerEncrypter()
    {}

    /**
     * Configure Serializable Closure signing for security.
     *
     * @return void
     */
    protected function registerSerializableClosureSecurityKey()
    {}

    /**
     * Parse the encryption key.
     *
     * @param  array  $config
     * @return string
     */
    protected function parseKey(array $config)
    {}

    /**
     * Extract the encryption key from the given configuration.
     *
     * @param  array  $config
     * @return string
     *
     * @throws \Illuminate\Encryption\MissingAppKeyException
     */
    protected function key(array $config)
    {}
}
