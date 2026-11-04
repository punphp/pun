<?php
namespace Hamcrest;

/*
 Copyright (c) 2009 hamcrest.org
 */

class MatcherAssert
{

    /**
     * Number of assertions performed.
     *
     * @var int
     */
    private static int $_count = 0;

    /**
     * Make an assertion and throw {@link Hamcrest\AssertionError} if it fails.
     *
     * The first parameter may optionally be a string identifying the assertion
     * to be included in the failure message.
     *
     * If the third parameter is not a matcher it is passed to
     * {@link Hamcrest\Core\IsEqual#equalTo} to create one.
     *
     * Example:
     * <pre>
     * // With an identifier
     * assertThat("apple flavour", $apple->flavour(), equalTo("tasty"));
     * // Without an identifier
     * assertThat($apple->flavour(), equalTo("tasty"));
     * // Evaluating a boolean expression
     * assertThat("some error", $a > $b);
     * assertThat($a > $b);
     * </pre>
     */
    public static function assertThat(/* $args ... */): void
    { func_get_args(); }

    /**
     * Returns the number of assertions performed.
     *
     * @return int
     */
    public static function getCount(): int
    {}

    /**
     * Resets the number of assertions performed to zero.
     */
    public static function resetCount(): void
    {}

    /**
     * Performs the actual assertion logic.
     *
     * If <code>$matcher</code> doesn't match <code>$actual</code>,
     * throws a {@link Hamcrest\AssertionError} with a description
     * of the failure along with the optional <code>$identifier</code>.
     *
     * @param string $identifier added to the message upon failure
     * @param mixed $actual value to compare against <code>$matcher</code>
     * @param \Hamcrest\Matcher $matcher applied to <code>$actual</code>
     * @throws AssertionError
     */
    private static function doAssert($identifier, $actual, Matcher $matcher): void
    {}
}
