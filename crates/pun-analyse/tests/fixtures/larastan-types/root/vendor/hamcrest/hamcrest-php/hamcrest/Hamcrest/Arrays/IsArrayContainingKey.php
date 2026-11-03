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
 * Matches if an array contains the specified key.
 */
class IsArrayContainingKey extends TypeSafeMatcher
{

    private Matcher $_keyMatcher;

    public function __construct(Matcher $keyMatcher)
    {}

    protected function matchesSafely($array): bool
    {}

    protected function describeMismatchSafely($array, Description $mismatchDescription): void
    {}

    public function describeTo(Description $description): void
    {}

    /**
     * Evaluates to true if any key in an array matches the given matcher.
     *
     * @param mixed $key as a {@link Hamcrest\Matcher} or a value.
     *
     * @return \Hamcrest\Arrays\IsArrayContainingKey
     * @factory hasKey
     */
    public static function hasKeyInArray($key): self
    {}
}
