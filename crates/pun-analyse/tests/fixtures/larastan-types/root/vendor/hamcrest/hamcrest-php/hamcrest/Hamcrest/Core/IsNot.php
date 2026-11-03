<?php
namespace Hamcrest\Core;

/*
 Copyright (c) 2009 hamcrest.org
 */
use Hamcrest\BaseMatcher;
use Hamcrest\Description;
use Hamcrest\Matcher;
use Hamcrest\Util;

/**
 * Calculates the logical negation of a matcher.
 */
class IsNot extends BaseMatcher
{

    private Matcher $_matcher;

    public function __construct(Matcher $matcher)
    {}

    public function matches($arg): bool
    {}

    public function describeTo(Description $description): void
    {}

    /**
     * Matches if value does not match $value.
     *
     * @factory
     * @param mixed $value
     */
    public static function not($value): self
    {}
}
