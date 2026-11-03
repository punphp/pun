<?php
namespace Hamcrest\Core;

/*
 Copyright (c) 2009 hamcrest.org
 */

use Hamcrest\BaseMatcher;
use Hamcrest\Description;
use Hamcrest\Matcher;

class CombinableMatcher extends BaseMatcher
{

    private Matcher $_matcher;

    public function __construct(Matcher $matcher)
    {}

    public function matches($item): bool
    {}

    public function describeTo(Description $description): void
    {}

    /** Diversion from Hamcrest-Java... Logical "and" not permitted */
    public function andAlso(Matcher $other): self
    {}

    /** Diversion from Hamcrest-Java... Logical "or" not permitted */
    public function orElse(Matcher $other): self
    {}

    /**
     * This is useful for fluently combining matchers that must both pass.
     * For example:
     * <pre>
     *   assertThat($string, both(containsString("a"))->andAlso(containsString("b")));
     * </pre>
     *
     * @factory
     */
    public static function both(Matcher $matcher): self
    {}

    /**
     * This is useful for fluently combining matchers where either may pass,
     * for example:
     * <pre>
     *   assertThat($string, either(containsString("a"))->orElse(containsString("b")));
     * </pre>
     *
     * @factory
     */
    public static function either(Matcher $matcher): self
    {}

    // -- Private Methods

    /**
     * @return list<Matcher>
     */
    private function _templatedListWith(Matcher $other): array
    {}
}
