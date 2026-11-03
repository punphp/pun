<?php
namespace Hamcrest\Arrays;

/*
 Copyright (c) 2009 hamcrest.org
 */
use Hamcrest\Description;
use Hamcrest\Matcher;
use Hamcrest\TypeSafeDiagnosingMatcher;
use Hamcrest\Util;

/**
 * Matches if an array contains a set of items satisfying nested matchers.
 */
class IsArrayContainingInAnyOrder extends TypeSafeDiagnosingMatcher
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

    protected function matchesSafelyWithDiagnosticDescription($array, Description $mismatchDescription): bool
    {}

    public function describeTo(Description $description): void
    {}

    /**
     * An array with elements that match the given matchers.
     *
     * @factory containsInAnyOrder ...
     */
    public static function arrayContainingInAnyOrder(/* args... */): self
    {}
}
