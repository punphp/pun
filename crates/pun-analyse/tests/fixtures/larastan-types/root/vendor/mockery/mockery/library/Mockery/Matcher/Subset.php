<?php

/**
 * Mockery (https://docs.mockery.io/en/stable/)
 *
 * @copyright https://github.com/mockery/mockery/blob/HEAD/COPYRIGHT.md
 * @license   https://github.com/mockery/mockery/blob/HEAD/LICENSE BSD 3-Clause License
 * @see       https://github.com/mockery/mockery for the canonical source repository
 */

namespace Mockery\Matcher;

use Override;

use ReturnTypeWillChange;

use function array_replace_recursive;
use function implode;
use function is_array;

class Subset extends MatcherAbstract
{
    private $expected;

    private $strict = true;

    /**
     * @param array $expected Expected subset of data
     * @param bool  $strict   Whether to run a strict or loose comparison
     */
    public function __construct(array $expected, $strict = true)
    {}

    /**
     * Return a string representation of this Matcher
     *
     * @return string
     */
    #[ReturnTypeWillChange]
    public function __toString()
    {}

    /**
     * @param  array  $expected Expected subset of data
     * @return Subset
     */
    public static function loose(array $expected)
    {}

    /**
     * Check if the actual value matches the expected.
     *
     * @param  mixed $actual
     * @return bool
     */
    #[Override]
    public function match(&$actual)
    {}

    /**
     * @param  array  $expected Expected subset of data
     * @return Subset
     */
    public static function strict(array $expected)
    {}

    /**
     * Recursively format an array into the string representation for this matcher
     *
     * @return string
     */
    protected function formatArray(array $array)
    {}
}
