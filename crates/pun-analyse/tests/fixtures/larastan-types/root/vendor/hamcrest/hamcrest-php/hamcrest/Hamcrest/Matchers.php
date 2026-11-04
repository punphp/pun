<?php

/*
 Copyright (c) 2009-2010 hamcrest.org
 */

// This file is generated from the static method @factory doctags.

namespace Hamcrest;

/**
 * A series of static factories for all hamcrest matchers.
 */
class Matchers
{

    /**
     * Evaluates to true only if each $matcher[$i] is satisfied by $array[$i].
     */
    public static function anArray(/* args... */): \Hamcrest\Arrays\IsArray
    { func_get_args(); }

    /**
     * Evaluates to true if any item in an array satisfies the given matcher.
     *
     * @param mixed $item as a {@link Hamcrest\Matcher} or a value.
     *
     * @return \Hamcrest\Arrays\IsArrayContaining
     */
    public static function hasItemInArray($item): \Hamcrest\Arrays\IsArrayContaining
    {}

    /**
     * Evaluates to true if any item in an array satisfies the given matcher.
     *
     * @param mixed $item as a {@link Hamcrest\Matcher} or a value.
     *
     * @return \Hamcrest\Arrays\IsArrayContaining
     */
    public static function hasValue($item): \Hamcrest\Arrays\IsArrayContaining
    {}

    /**
     * An array with elements that match the given matchers.
     */
    public static function arrayContainingInAnyOrder(/* args... */): \Hamcrest\Arrays\IsArrayContainingInAnyOrder
    { func_get_args(); }

    /**
     * An array with elements that match the given matchers.
     */
    public static function containsInAnyOrder(/* args... */): \Hamcrest\Arrays\IsArrayContainingInAnyOrder
    { func_get_args(); }

    /**
     * An array with elements that match the given matchers in the same order.
     */
    public static function arrayContaining(/* args... */): \Hamcrest\Arrays\IsArrayContainingInOrder
    { func_get_args(); }

    /**
     * An array with elements that match the given matchers in the same order.
     */
    public static function contains(/* args... */): \Hamcrest\Arrays\IsArrayContainingInOrder
    { func_get_args(); }

    /**
     * Evaluates to true if any key in an array matches the given matcher.
     *
     * @param mixed $key as a {@link Hamcrest\Matcher} or a value.
     *
     * @return \Hamcrest\Arrays\IsArrayContainingKey
     */
    public static function hasKeyInArray($key): \Hamcrest\Arrays\IsArrayContainingKey
    {}

    /**
     * Evaluates to true if any key in an array matches the given matcher.
     *
     * @param mixed $key as a {@link Hamcrest\Matcher} or a value.
     *
     * @return \Hamcrest\Arrays\IsArrayContainingKey
     */
    public static function hasKey($key): \Hamcrest\Arrays\IsArrayContainingKey
    {}

    /**
     * Test if an array has both an key and value in parity with each other.
     *
     * @param mixed $key
     * @param mixed $value
     */
    public static function hasKeyValuePair($key, $value): \Hamcrest\Arrays\IsArrayContainingKeyValuePair
    {}

    /**
     * Test if an array has both an key and value in parity with each other.
     *
     * @param mixed $key
     * @param mixed $value
     */
    public static function hasEntry($key, $value): \Hamcrest\Arrays\IsArrayContainingKeyValuePair
    {}

    /**
     * Does array size satisfy a given matcher?
     *
     * @param \Hamcrest\Matcher|int $size as a {@link Hamcrest\Matcher} or a value.
     *
     * @return \Hamcrest\Arrays\IsArrayWithSize
     */
    public static function arrayWithSize($size): \Hamcrest\Arrays\IsArrayWithSize
    {}

    /**
     * Matches an empty array.
     */
    public static function emptyArray(): \Hamcrest\Core\DescribedAs
    {}

    /**
     * Matches an empty array.
     */
    public static function nonEmptyArray(): \Hamcrest\Core\DescribedAs
    {}

    /**
     * Returns true if traversable is empty.
     */
    public static function emptyTraversable(): \Hamcrest\Collection\IsEmptyTraversable
    {}

    /**
     * Returns true if traversable is not empty.
     */
    public static function nonEmptyTraversable(): \Hamcrest\Collection\IsEmptyTraversable
    {}

    /**
     * Does traversable size satisfy a given matcher?
     *
     * @param mixed $size
     */
    public static function traversableWithSize($size): \Hamcrest\Collection\IsTraversableWithSize
    {}

    /**
     * Evaluates to true only if ALL of the passed in matchers evaluate to true.
     */
    public static function allOf(/* args... */): \Hamcrest\Core\AllOf
    { func_get_args(); }

    /**
     * Evaluates to true if ANY of the passed in matchers evaluate to true.
     */
    public static function anyOf(/* args... */): \Hamcrest\Core\AnyOf
    { func_get_args(); }

    /**
     * Evaluates to false if ANY of the passed in matchers evaluate to true.
     */
    public static function noneOf(/* args... */): \Hamcrest\Core\IsNot
    { func_get_args(); }

    /**
     * This is useful for fluently combining matchers that must both pass.
     * For example:
     * <pre>
     *   assertThat($string, both(containsString("a"))->andAlso(containsString("b")));
     * </pre>
     */
    public static function both(\Hamcrest\Matcher $matcher): \Hamcrest\Core\CombinableMatcher
    {}

    /**
     * This is useful for fluently combining matchers where either may pass,
     * for example:
     * <pre>
     *   assertThat($string, either(containsString("a"))->orElse(containsString("b")));
     * </pre>
     */
    public static function either(\Hamcrest\Matcher $matcher): \Hamcrest\Core\CombinableMatcher
    {}

    /**
     * Wraps an existing matcher and overrides the description when it fails.
     */
    public static function describedAs(/* args... */): \Hamcrest\Core\DescribedAs
    { func_get_args(); }

    /**
     * @param \Hamcrest\Matcher $itemMatcher
     *   A matcher to apply to every element in an array.
     *
     * @return \Hamcrest\Core\Every
     *   Evaluates to TRUE for a collection in which every item matches $itemMatcher
     */
    public static function everyItem(\Hamcrest\Matcher $itemMatcher): \Hamcrest\Core\Every
    {}

    /**
     * Creates a matcher that matches any examined object whose <code>toString</code> or
     * <code>__toString()</code> method returns a value equalTo the specified string.
     *
     * @param mixed $matcher
     */
    public static function hasToString($matcher): \Hamcrest\Core\HasToString
    {}

    /**
     * Decorates another Matcher, retaining the behavior but allowing tests
     * to be slightly more expressive.
     *
     * For example:  assertThat($cheese, equalTo($smelly))
     *          vs.  assertThat($cheese, is(equalTo($smelly)))
     *
     * @param mixed $value
     */
    public static function is($value): \Hamcrest\Core\Is
    {}

    /**
     * This matcher always evaluates to true.
     *
     * @param string $description A meaningful string used when describing itself.
     */
    public static function anything(string $description = 'ANYTHING'): \Hamcrest\Core\IsAnything
    {}

    /**
     * Test if the value is an array containing this matcher.
     *
     * Example:
     * <pre>
     * assertThat(array('a', 'b'), hasItem(equalTo('b')));
     * //Convenience defaults to equalTo()
     * assertThat(array('a', 'b'), hasItem('b'));
     * </pre>
     */
    public static function hasItem(/* args... */): \Hamcrest\Core\IsCollectionContaining
    { func_get_args(); }

    /**
     * Test if the value is an array containing elements that match all of these
     * matchers.
     *
     * Example:
     * <pre>
     * assertThat(array('a', 'b', 'c'), hasItems(equalTo('a'), equalTo('b')));
     * </pre>
     */
    public static function hasItems(/* args... */): \Hamcrest\Core\AllOf
    { func_get_args(); }

    /**
     * Is the value equal to another value, as tested by the use of the "=="
     * comparison operator?
     *
     * @param mixed $item
     */
    public static function equalTo($item): \Hamcrest\Core\IsEqual
    {}

    /**
     * Tests of the value is identical to $value as tested by the "===" operator.
     *
     * @param mixed $value
     */
    public static function identicalTo($value): \Hamcrest\Core\IsIdentical
    {}

    /**
     * Is the value an instance of a particular type?
     * This version assumes no relationship between the required type and
     * the signature of the method that sets it up, for example in
     * <code>assertThat($anObject, anInstanceOf('Thing'));</code>
     */
    public static function anInstanceOf(string $theClass): \Hamcrest\Core\IsInstanceOf
    {}

    /**
     * Is the value an instance of a particular type?
     * This version assumes no relationship between the required type and
     * the signature of the method that sets it up, for example in
     * <code>assertThat($anObject, anInstanceOf('Thing'));</code>
     */
    public static function any(string $theClass): \Hamcrest\Core\IsInstanceOf
    {}

    /**
     * Matches if value does not match $value.
     *
     * @param mixed $value
     */
    public static function not($value): \Hamcrest\Core\IsNot
    {}

    /**
     * Matches if value is null.
     */
    public static function nullValue(): \Hamcrest\Core\IsNull
    {}

    /**
     * Matches if value is not null.
     */
    public static function notNullValue(): \Hamcrest\Core\IsNot
    {}

    /**
     * Creates a new instance of IsSame.
     *
     * @param mixed $object
     *   The predicate evaluates to true only when the argument is
     *   this object.
     *
     * @return \Hamcrest\Core\IsSame
     */
    public static function sameInstance($object): \Hamcrest\Core\IsSame
    {}

    /**
     * Is the value a particular built-in type?
     *
     * @param string $theType
     */
    public static function typeOf(string $theType): \Hamcrest\Core\IsTypeOf
    {}

    /**
     * Matches if value (class, object, or array) has named $property.
     *
     * @param mixed $property
     */
    public static function set($property): \Hamcrest\Core\Set
    {}

    /**
     * Matches if value (class, object, or array) does not have named $property.
     *
     * @param mixed $property
     */
    public static function notSet($property): \Hamcrest\Core\Set
    {}

    /**
     * Matches if value is a number equal to $value within some range of
     * acceptable error $delta.
     *
     * @param mixed $value
     * @param mixed $delta
     */
    public static function closeTo($value, $delta): \Hamcrest\Number\IsCloseTo
    {}

    /**
     * The value is not > $value, nor < $value.
     *
     * @param mixed $value
     */
    public static function comparesEqualTo($value): \Hamcrest\Number\OrderingComparison
    {}

    /**
     * The value is > $value.
     *
     * @param mixed $value
     */
    public static function greaterThan($value): \Hamcrest\Number\OrderingComparison
    {}

    /**
     * The value is >= $value.
     *
     * @param mixed $value
     */
    public static function greaterThanOrEqualTo($value): \Hamcrest\Number\OrderingComparison
    {}

    /**
     * The value is >= $value.
     *
     * @param mixed $value
     */
    public static function atLeast($value): \Hamcrest\Number\OrderingComparison
    {}

    /**
     * The value is < $value.
     *
     * @param mixed $value
     */
    public static function lessThan($value): \Hamcrest\Number\OrderingComparison
    {}

    /**
     * The value is <= $value.
     *
     * @param mixed $value
     */
    public static function lessThanOrEqualTo($value): \Hamcrest\Number\OrderingComparison
    {}

    /**
     * The value is <= $value.
     *
     * @param mixed $value
     */
    public static function atMost($value): \Hamcrest\Number\OrderingComparison
    {}

    /**
     * Matches if value is a zero-length string.
     */
    public static function isEmptyString(): \Hamcrest\Text\IsEmptyString
    {}

    /**
     * Matches if value is a zero-length string.
     */
    public static function emptyString(): \Hamcrest\Text\IsEmptyString
    {}

    /**
     * Matches if value is null or a zero-length string.
     */
    public static function isEmptyOrNullString(): \Hamcrest\Core\AnyOf
    {}

    /**
     * Matches if value is null or a zero-length string.
     */
    public static function nullOrEmptyString(): \Hamcrest\Core\AnyOf
    {}

    /**
     * Matches if value is a non-zero-length string.
     */
    public static function isNonEmptyString(): \Hamcrest\Text\IsEmptyString
    {}

    /**
     * Matches if value is a non-zero-length string.
     */
    public static function nonEmptyString(): \Hamcrest\Text\IsEmptyString
    {}

    /**
     * Matches if value is a string equal to $string, regardless of the case.
     *
     * @param mixed $string
     */
    public static function equalToIgnoringCase($string): \Hamcrest\Text\IsEqualIgnoringCase
    {}

    /**
     * Matches if value is a string equal to $string, regardless of whitespace.
     *
     * @param mixed $string
     */
    public static function equalToIgnoringWhiteSpace($string): \Hamcrest\Text\IsEqualIgnoringWhiteSpace
    {}

    /**
     * Matches if value is a string that matches regular expression $pattern.
     *
     * @param mixed $pattern
     */
    public static function matchesPattern($pattern): \Hamcrest\Text\MatchesPattern
    {}

    /**
     * Matches if value is a string that contains $substring.
     *
     * @param mixed $substring
     */
    public static function containsString($substring): \Hamcrest\Text\StringContains
    {}

    /**
     * Matches if value is a string that contains $substring regardless of the case.
     *
     * @param mixed $substring
     */
    public static function containsStringIgnoringCase($substring): \Hamcrest\Text\StringContainsIgnoringCase
    {}

    /**
     * Matches if value contains $substrings in a constrained order.
     */
    public static function stringContainsInOrder(/* args... */): \Hamcrest\Text\StringContainsInOrder
    { func_get_args(); }

    /**
     * Matches if value is a string that ends with $substring.
     *
     * @param mixed $substring
     */
    public static function endsWith($substring): \Hamcrest\Text\StringEndsWith
    {}

    /**
     * Matches if value is a string that starts with $substring.
     *
     * @param mixed $substring
     */
    public static function startsWith($substring): \Hamcrest\Text\StringStartsWith
    {}

    /**
     * Is the value an array?
     */
    public static function arrayValue(): \Hamcrest\Type\IsArray
    {}

    /**
     * Is the value a boolean?
     */
    public static function booleanValue(): \Hamcrest\Type\IsBoolean
    {}

    /**
     * Is the value a boolean?
     */
    public static function boolValue(): \Hamcrest\Type\IsBoolean
    {}

    /**
     * Is the value callable?
     */
    public static function callableValue(): \Hamcrest\Type\IsCallable
    {}

    /**
     * Is the value a float/double?
     */
    public static function doubleValue(): \Hamcrest\Type\IsDouble
    {}

    /**
     * Is the value a float/double?
     */
    public static function floatValue(): \Hamcrest\Type\IsDouble
    {}

    /**
     * Is the value an integer?
     */
    public static function integerValue(): \Hamcrest\Type\IsInteger
    {}

    /**
     * Is the value an integer?
     */
    public static function intValue(): \Hamcrest\Type\IsInteger
    {}

    /**
     * Is the value a numeric?
     */
    public static function numericValue(): \Hamcrest\Type\IsNumeric
    {}

    /**
     * Is the value an object?
     */
    public static function objectValue(): \Hamcrest\Type\IsObject
    {}

    /**
     * Is the value an object?
     */
    public static function anObject(): \Hamcrest\Type\IsObject
    {}

    /**
     * Is the value a resource?
     */
    public static function resourceValue(): \Hamcrest\Type\IsResource
    {}

    /**
     * Is the value a scalar (boolean, integer, double, or string)?
     */
    public static function scalarValue(): \Hamcrest\Type\IsScalar
    {}

    /**
     * Is the value a string?
     */
    public static function stringValue(): \Hamcrest\Type\IsString
    {}

    /**
     * Wraps <code>$matcher</code> with {@link Hamcrest\Core\IsEqual)
     * if it's not a matcher and the XPath in <code>count()</code>
     * if it's an integer.
     *
     * @param string $xpath
     * @param null|Matcher|int|mixed $matcher
     */
    public static function hasXPath(string $xpath, $matcher = null): \Hamcrest\Xml\HasXPath
    {}
}
