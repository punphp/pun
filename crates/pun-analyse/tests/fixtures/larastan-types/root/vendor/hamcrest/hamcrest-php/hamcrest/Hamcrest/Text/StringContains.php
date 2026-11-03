<?php
namespace Hamcrest\Text;

/*
 Copyright (c) 2009 hamcrest.org
 */

/**
 * Tests if the argument is a string that contains a substring.
 */
class StringContains extends SubstringMatcher
{
    /**
     * @param mixed $substring
     */
    public function __construct($substring)
    {}

    public function ignoringCase(): StringContainsIgnoringCase
    {}

    /**
     * Matches if value is a string that contains $substring.
     *
     * @factory
     * @param mixed $substring
     */
    public static function containsString($substring): self
    {}

    // -- Protected Methods

    protected function evalSubstringOf(string $item): bool
    {}

    protected function relationship(): string
    {}
}
