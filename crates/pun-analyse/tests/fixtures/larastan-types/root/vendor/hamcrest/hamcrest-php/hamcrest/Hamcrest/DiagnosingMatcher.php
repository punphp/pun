<?php
namespace Hamcrest;

/*
 Copyright (c) 2009 hamcrest.org
 */

/**
 * Official documentation for this class is missing.
 */
abstract class DiagnosingMatcher extends BaseMatcher
{

    final public function matches($item): bool
    {}

    public function describeMismatch($item, Description $mismatchDescription): void
    {}

    /**
     * @param mixed $item
     */
    abstract protected function matchesWithDiagnosticDescription($item, Description $mismatchDescription): bool;
}
