<?php

namespace Illuminate\Hashing;

use Error;
use Illuminate\Contracts\Hashing\Hasher as HasherContract;
use RuntimeException;

class ArgonHasher extends AbstractHasher implements HasherContract
{
    /**
     * The default memory cost factor.
     *
     * @var int
     */
    protected $memory = 1024;

    /**
     * The default time cost factor.
     *
     * @var int
     */
    protected $time = 2;

    /**
     * The default threads factor.
     *
     * @var int
     */
    protected $threads = 2;

    /**
     * Indicates whether to perform an algorithm check.
     *
     * @var bool
     */
    protected $verifyAlgorithm = false;

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
     */
    public function make(#[\SensitiveParameter] $value, array $options = [])
    {}

    /**
     * Get the algorithm that should be used for hashing.
     *
     * @return string
     */
    protected function algorithm()
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
     * Set the default password memory factor.
     *
     * @param  int  $memory
     * @return $this
     */
    public function setMemory(int $memory)
    {}

    /**
     * Set the default password timing factor.
     *
     * @param  int  $time
     * @return $this
     */
    public function setTime(int $time)
    {}

    /**
     * Set the default password threads factor.
     *
     * @param  int  $threads
     * @return $this
     */
    public function setThreads(int $threads)
    {}

    /**
     * Extract the memory cost value from the options array.
     *
     * @param  array  $options
     * @return int
     */
    protected function memory(array $options)
    {}

    /**
     * Extract the time cost value from the options array.
     *
     * @param  array  $options
     * @return int
     */
    protected function time(array $options)
    {}

    /**
     * Extract the thread's value from the options array.
     *
     * @param  array  $options
     * @return int
     */
    protected function threads(array $options)
    {}
}
