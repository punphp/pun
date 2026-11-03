<?php
namespace Hamcrest\Arrays;

/**
 * Tests for the presence of both a key and value inside an array.
 */
use Hamcrest\Description;
use Hamcrest\Matcher;
use Hamcrest\TypeSafeMatcher;
use Hamcrest\Util;

/**
 * @namespace
 */

class IsArrayContainingKeyValuePair extends TypeSafeMatcher
{
    private Matcher $_keyMatcher;
    private Matcher $_valueMatcher;

    public function __construct(Matcher $keyMatcher, Matcher $valueMatcher)
    {}

    protected function matchesSafely($array): bool
    {}

    protected function describeMismatchSafely($array, Description $mismatchDescription): void
    {}

    public function describeTo(Description $description): void
    {}

    /**
     * Test if an array has both an key and value in parity with each other.
     *
     * @factory hasEntry
     * @param mixed $key
     * @param mixed $value
     */
    public static function hasKeyValuePair($key, $value): self
    {}
}
