<?php
namespace Hamcrest\Text;

/*
 Copyright (c) 2009 hamcrest.org
 */
use Hamcrest\Description;
use Hamcrest\TypeSafeMatcher;

/**
 * Tests if the value contains a series of substrings in a constrained order.
 */
class StringContainsInOrder extends TypeSafeMatcher
{
    /**
     * @var array<string>
     */
    private array $_substrings;

    /**
     * @param array<string> $substrings
     */
    public function __construct(array $substrings)
    {}

    protected function matchesSafely($item): bool
    {}

    protected function describeMismatchSafely($item, Description $mismatchDescription): void
    {}

    public function describeTo(Description $description): void
    {}

    /**
     * Matches if value contains $substrings in a constrained order.
     *
     * @factory ...
     */
    public static function stringContainsInOrder(/* args... */): self
    { func_get_args(); }
}
