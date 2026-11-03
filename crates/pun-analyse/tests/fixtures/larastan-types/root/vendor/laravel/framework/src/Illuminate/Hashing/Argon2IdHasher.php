<?php

namespace Illuminate\Hashing;

use RuntimeException;

class Argon2IdHasher extends ArgonHasher
{
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
     * Verify the hashed value's algorithm.
     *
     * @param  string  $hashedValue
     * @return bool
     */
    protected function isUsingCorrectAlgorithm($hashedValue)
    {}

    /**
     * Get the algorithm that should be used for hashing.
     *
     * @return string
     */
    protected function algorithm()
    {}
}
