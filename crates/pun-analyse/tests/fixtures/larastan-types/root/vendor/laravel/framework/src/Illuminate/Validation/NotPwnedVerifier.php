<?php

namespace Illuminate\Validation;

use Exception;
use Illuminate\Contracts\Validation\UncompromisedVerifier;
use Illuminate\Support\Stringable;

class NotPwnedVerifier implements UncompromisedVerifier
{
    /**
     * The HTTP factory instance.
     *
     * @var \Illuminate\Http\Client\Factory
     */
    protected $factory;

    /**
     * The number of seconds the request can run before timing out.
     *
     * @var int
     */
    protected $timeout;

    /**
     * Create a new uncompromised verifier.
     *
     * @param  \Illuminate\Http\Client\Factory  $factory
     * @param  int|null  $timeout
     */
    public function __construct($factory, $timeout = null)
    {}

    /**
     * Verify that the given data has not been compromised in public breaches.
     *
     * @param  array  $data
     * @return bool
     */
    public function verify($data)
    {}

    /**
     * Get the hash and its first 5 chars.
     *
     * @param  string  $value
     * @return array
     */
    protected function getHash($value)
    {}

    /**
     * Search by the given hash prefix and returns all occurrences of leaked passwords.
     *
     * @param  string  $hashPrefix
     * @return \Illuminate\Support\Collection
     */
    protected function search($hashPrefix)
    {}
}
