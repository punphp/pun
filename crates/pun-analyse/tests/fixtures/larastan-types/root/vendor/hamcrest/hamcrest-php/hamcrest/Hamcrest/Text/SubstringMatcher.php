<?php
namespace Hamcrest\Text;

/*
 Copyright (c) 2009 hamcrest.org
 */

use Hamcrest\Description;
use Hamcrest\TypeSafeMatcher;

abstract class SubstringMatcher extends TypeSafeMatcher
{

    /**
     * @var mixed
     */
    protected $_substring;

    /**
     * @param mixed $substring
     */
    public function __construct($substring)
    {}

    protected function matchesSafely($item): bool
    {}

    protected function describeMismatchSafely($item, Description $mismatchDescription): void
    {}

    public function describeTo(Description $description): void
    {}

    abstract protected function evalSubstringOf(string $string): bool;

    abstract protected function relationship(): string;
}
