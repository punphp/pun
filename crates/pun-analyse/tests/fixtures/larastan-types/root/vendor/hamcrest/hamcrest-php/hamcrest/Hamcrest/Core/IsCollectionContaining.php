<?php
namespace Hamcrest\Core;

/*
 Copyright (c) 2009 hamcrest.org
 */
use Hamcrest\Description;
use Hamcrest\Matcher;
use Hamcrest\TypeSafeMatcher;
use Hamcrest\Util;

/**
 * Tests if an array contains values that match one or more Matchers.
 */
class IsCollectionContaining extends TypeSafeMatcher
{

    private Matcher $_elementMatcher;

    public function __construct(Matcher $elementMatcher)
    {}

    protected function matchesSafely($items): bool
    {}

    protected function describeMismatchSafely($items, Description $mismatchDescription): void
    {}

    public function describeTo(Description $description): void
    {}

    /**
     * Test if the value is an array containing this matcher.
     *
     * Example:
     * <pre>
     * assertThat(array('a', 'b'), hasItem(equalTo('b')));
     * //Convenience defaults to equalTo()
     * assertThat(array('a', 'b'), hasItem('b'));
     * </pre>
     *
     * @factory ...
     */
    public static function hasItem(): self
    { func_get_args(); }

    /**
     * Test if the value is an array containing elements that match all of these
     * matchers.
     *
     * Example:
     * <pre>
     * assertThat(array('a', 'b', 'c'), hasItems(equalTo('a'), equalTo('b')));
     * </pre>
     *
     * @factory ...
     */
    public static function hasItems(/* args... */): AllOf
    { func_get_args(); }
}
