<?php
namespace Hamcrest\Number;

/*
 Copyright (c) 2009 hamcrest.org
 */
use Hamcrest\Description;
use Hamcrest\TypeSafeMatcher;

/**
 * Is the value a number equal to a value within some range of
 * acceptable error?
 */
class IsCloseTo extends TypeSafeMatcher
{

    /**
     * @var mixed
     */
    private $_value;
    /**
     * @var mixed
     */
    private $_delta;

    /**
     * @param mixed $value
     * @param mixed $delta
     */
    public function __construct($value, $delta)
    {}

    protected function matchesSafely($item): bool
    {}

    protected function describeMismatchSafely($item, Description $mismatchDescription): void
    {}

    public function describeTo(Description $description): void
    {}

    /**
     * Matches if value is a number equal to $value within some range of
     * acceptable error $delta.
     *
     * @factory
     * @param mixed $value
     * @param mixed $delta
     */
    public static function closeTo($value, $delta): self
    {}

    // -- Private Methods

    /**
     * @param mixed $item
     * @return int|float
     */
    private function _actualDelta($item)
    {}
}
