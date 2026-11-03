<?php

namespace Illuminate\Hashing;

use Error;
use Illuminate\Contracts\Hashing\Hasher as HasherContract;
use InvalidArgumentException;
use RuntimeException;

class BcryptHasher extends AbstractHasher implements HasherContract
{
    /**
     * The default cost factor.
     *
     * @var int
     */
    protected $rounds = 12;

    /**
     * Indicates whether to perform an algorithm check.
     *
     * @var bool
     */
    protected $verifyAlgorithm = false;

    /**
     * The maximum allowed length of strings that can be hashed.
     *
     * @var int|null
     */
    protected $limit;

    /**
     * Create a new hasher instance.
     *
     * @param  array  $options
     */
    public function __construct(array $options = [])
    {}

    /**
     * Hash the given value.
     *
     * @param  string  $value
     * @param  array  $options
     * @return string
     *
     * @throws \RuntimeException
     * @throws \InvalidArgumentException
     */
    public function make(#[\SensitiveParameter] $value, array $options = [])
    {}

    /**
     * Check the given plain value against a hash.
     *
     * @param  string  $value
     * @param  string|null  $hashedValue
     * @param  array  $options
     * @return bool
     *
     * @throws \RuntimeException
     */
    public function check(#[\SensitiveParameter] $value, $hashedValue, array $options = [])
    {}

    /**
     * Check if the given hash has been hashed using the given options.
     *
     * @param  string  $hashedValue
     * @param  array  $options
     * @return bool
     */
    public function needsRehash($hashedValue, array $options = [])
    {}

    /**
     * Verifies that the configuration is less than or equal to what is configured.
     *
     * @internal
     */
    public function verifyConfiguration($value)
    {}

    /**
     * Verify the hashed value's algorithm.
     *
     * @param  string  $hashedValue
     * @return bool
     */
    protected function isUsingCorrectAlgorithm($hashedValue)
    {}

    /**
     * Verify the hashed value's options.
     *
     * @param  string  $hashedValue
     * @return bool
     */
    protected function isUsingValidOptions($hashedValue)
    {}

    /**
     * Set the default password work factor.
     *
     * @param  int  $rounds
     * @return $this
     */
    public function setRounds($rounds)
    {}

    /**
     * Extract the cost value from the options array.
     *
     * @param  array  $options
     * @return int
     */
    protected function cost(array $options = [])
    {}
}
