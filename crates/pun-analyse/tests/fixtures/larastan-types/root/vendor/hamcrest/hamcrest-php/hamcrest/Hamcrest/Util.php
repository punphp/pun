<?php
namespace Hamcrest;

/*
 Copyright (c) 2012 hamcrest.org
 */

/**
 * Contains utility methods for handling Hamcrest matchers.
 *
 * @see Hamcrest\Matcher
 */
class Util
{
    public static function registerGlobalFunctions(): void
    {}

    /**
     * Wraps the item with an IsEqual matcher if it isn't a matcher already.
     *
     * @param mixed $item matcher or any value
     * @return \Hamcrest\Matcher
     */
    public static function wrapValueWithIsEqual($item)
    {}

    /**
     * Throws an exception if any item in $matchers is not a Hamcrest\Matcher.
     *
     * @param array<mixed> $matchers expected to contain only matchers
     * @throws \InvalidArgumentException if any item is not a matcher
     */
    public static function checkAllAreMatchers(array $matchers): void
    {}

    /**
     * Returns a copy of $items where each non-Matcher item is replaced by
     * a Hamcrest\Core\IsEqual matcher for the item. If the first and only item
     * is an array, it is used as the $items array to support the old style
     * of passing an array as the sole argument to a matcher.
     *
     * @param array<mixed> $items contains items and matchers
     * @return array<Matcher> all items are
     */
    public static function createMatcherArray(array $items)
    {}
}
