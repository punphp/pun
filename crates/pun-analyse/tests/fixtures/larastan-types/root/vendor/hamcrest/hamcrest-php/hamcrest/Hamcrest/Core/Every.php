<?php
namespace Hamcrest\Core;

/*
 Copyright (c) 2009 hamcrest.org
 */

use Hamcrest\Description;
use Hamcrest\Matcher;
use Hamcrest\TypeSafeDiagnosingMatcher;

class Every extends TypeSafeDiagnosingMatcher
{

    private Matcher $_matcher;

    public function __construct(Matcher $matcher)
    {}

    protected function matchesSafelyWithDiagnosticDescription($items, Description $mismatchDescription): bool
    {}

    public function describeTo(Description $description): void
    {}

    /**
     * @param \Hamcrest\Matcher $itemMatcher
     *   A matcher to apply to every element in an array.
     *
     * @return \Hamcrest\Core\Every
     *   Evaluates to TRUE for a collection in which every item matches $itemMatcher
     *
     * @factory
     */
    public static function everyItem(Matcher $itemMatcher): self
    {}
}
