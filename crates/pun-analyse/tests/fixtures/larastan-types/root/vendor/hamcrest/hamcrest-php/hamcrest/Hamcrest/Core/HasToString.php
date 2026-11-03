<?php
namespace Hamcrest\Core;

/*
 Copyright (c) 2009 hamcrest.org
 */
use Hamcrest\Description;
use Hamcrest\FeatureMatcher;
use Hamcrest\Matcher;
use Hamcrest\Util;

/**
 * A Matcher that checks the output of the <code>toString()</code> or <code>__toString()</code> method.
 */
class HasToString extends FeatureMatcher
{

    public function __construct(Matcher $toStringMatcher)
    {}

    public function matchesSafelyWithDiagnosticDescription($actual, Description $mismatchDescription): bool
    {}

    protected function featureValueOf($actual)
    {}

    /**
     * Creates a matcher that matches any examined object whose <code>toString</code> or
     * <code>__toString()</code> method returns a value equalTo the specified string.
     *
     * @factory
     * @param mixed $matcher
     */
    public static function hasToString($matcher): self
    {}
}
