<?php
namespace Hamcrest\Number;

/*
 Copyright (c) 2009 hamcrest.org
 */

use Hamcrest\Description;
use Hamcrest\TypeSafeMatcher;

class OrderingComparison extends TypeSafeMatcher
{

    /**
     * @var mixed
     */
    private $_value;
    /**
     * @var mixed
     */
    private $_minCompare;
    /**
     * @var mixed
     */
    private $_maxCompare;

    /**
     * @param mixed $value
     * @param mixed $maxCompare
     * @param mixed $minCompare
     */
    public function __construct($value, $minCompare, $maxCompare)
    {}

    protected function matchesSafely($other): bool
    {}

    protected function describeMismatchSafely($item, Description $mismatchDescription): void
    {}

    public function describeTo(Description $description): void
    {}

    /**
     * The value is not > $value, nor < $value.
     *
     * @factory
     * @param mixed $value
     */
    public static function comparesEqualTo($value): self
    {}

    /**
     * The value is > $value.
     *
     * @factory
     * @param mixed $value
     */
    public static function greaterThan($value): self
    {}

    /**
     * The value is >= $value.
     *
     * @factory atLeast
     * @param mixed $value
     */
    public static function greaterThanOrEqualTo($value): self
    {}

    /**
     * The value is < $value.
     *
     * @factory
     * @param mixed $value
     */
    public static function lessThan($value): self
    {}

    /**
     * The value is <= $value.
     *
     * @factory atMost
     * @param mixed $value
     */
    public static function lessThanOrEqualTo($value): self
    {}

    // -- Private Methods

    /**
     * @param mixed $left
     * @param mixed $right
     */
    private function _compare($left, $right): int
    {}

    /**
     * @param mixed $compare
     */
    private function _comparison($compare): string
    {}
}
