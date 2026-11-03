<?php
namespace Hamcrest\Arrays;

/*
 Copyright (c) 2009 hamcrest.org
 */
use Hamcrest\Description;
use Hamcrest\Matcher;
use Hamcrest\TypeSafeMatcher;
use Hamcrest\Util;

/**
 * Matches if an array contains an item satisfying a nested matcher.
 */
class IsArrayContaining extends TypeSafeMatcher
{

    private Matcher $_elementMatcher;

    public function __construct(Matcher $elementMatcher)
    {}

    protected function matchesSafely($array): bool
    {}

    protected function describeMismatchSafely($array, Description $mismatchDescription): void
    {}

    public function describeTo(Description $description): void
    {}

    /**
     * Evaluates to true if any item in an array satisfies the given matcher.
     *
     * @param mixed $item as a {@link Hamcrest\Matcher} or a value.
     *
     * @return \Hamcrest\Arrays\IsArrayContaining
     * @factory hasValue
     */
    public static function hasItemInArray($item): self
    {}
}
