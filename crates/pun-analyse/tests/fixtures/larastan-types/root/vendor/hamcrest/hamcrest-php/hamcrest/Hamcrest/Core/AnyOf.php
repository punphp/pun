<?php
namespace Hamcrest\Core;

/*
 Copyright (c) 2009 hamcrest.org
 */
use Hamcrest\Description;
use Hamcrest\Matcher;
use Hamcrest\Util;

/**
 * Calculates the logical disjunction of multiple matchers. Evaluation is
 * shortcut, so subsequent matchers are not called if an earlier matcher
 * returns <code>true</code>.
 */
class AnyOf extends ShortcutCombination
{

    /**
     * @param array<Matcher> $matchers
     */
    public function __construct(array $matchers)
    {}

    public function matches($item): bool
    {}

    public function describeTo(Description $description): void
    {}

    /**
     * Evaluates to true if ANY of the passed in matchers evaluate to true.
     *
     * @factory ...
     */
    public static function anyOf(/* args... */): self
    {}

    /**
     * Evaluates to false if ANY of the passed in matchers evaluate to true.
     *
     * @factory ...
     */
    public static function noneOf(/* args... */): IsNot
    {}
}
