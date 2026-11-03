<?php
namespace Hamcrest\Arrays;

/*
 Copyright (c) 2009 hamcrest.org
 */

// NOTE: This class is not exactly a direct port of Java's since Java handles
//       arrays quite differently than PHP

// TODO: Allow this to take matchers or values within the array
use Hamcrest\Description;
use Hamcrest\Matcher;
use Hamcrest\TypeSafeMatcher;
use Hamcrest\Util;

/**
 * Matcher for array whose elements satisfy a sequence of matchers.
 * The array size must equal the number of element matchers.
 */
class IsArray extends TypeSafeMatcher
{

    /**
     * @var array<Matcher>
     */
    private array $_elementMatchers;

    /**
     * @param array<Matcher> $elementMatchers
     */
    public function __construct(array $elementMatchers)
    {}

    protected function matchesSafely($array): bool
    {}

    protected function describeMismatchSafely($actual, Description $mismatchDescription): void
    {}

    public function describeTo(Description $description): void
    {}

    /**
     * Evaluates to true only if each $matcher[$i] is satisfied by $array[$i].
     *
     * @factory ...
     */
    public static function anArray(/* args... */): self
    {}

    // -- Protected Methods

    protected function descriptionStart(): string
    {}

    protected function descriptionSeparator(): string
    {}

    protected function descriptionEnd(): string
    {}
}
