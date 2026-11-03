<?php declare(strict_types=1);
/*
 * This file is part of PHPUnit.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace PHPUnit\Framework;

use function function_exists;
use ArrayAccess;
use Countable;
use PHPUnit\Event\Facade as EventFacade;
use PHPUnit\Framework\Constraint\ArrayHasKey;
use PHPUnit\Framework\Constraint\Callback;
use PHPUnit\Framework\Constraint\Constraint;
use PHPUnit\Framework\Constraint\Count;
use PHPUnit\Framework\Constraint\DirectoryExists;
use PHPUnit\Framework\Constraint\FileExists;
use PHPUnit\Framework\Constraint\GreaterThan;
use PHPUnit\Framework\Constraint\IsAnything;
use PHPUnit\Framework\Constraint\IsEmpty;
use PHPUnit\Framework\Constraint\IsEqual;
use PHPUnit\Framework\Constraint\IsEqualCanonicalizing;
use PHPUnit\Framework\Constraint\IsEqualIgnoringCase;
use PHPUnit\Framework\Constraint\IsEqualWithDelta;
use PHPUnit\Framework\Constraint\IsFalse;
use PHPUnit\Framework\Constraint\IsFinite;
use PHPUnit\Framework\Constraint\IsIdentical;
use PHPUnit\Framework\Constraint\IsInfinite;
use PHPUnit\Framework\Constraint\IsInstanceOf;
use PHPUnit\Framework\Constraint\IsJson;
use PHPUnit\Framework\Constraint\IsList;
use PHPUnit\Framework\Constraint\IsNan;
use PHPUnit\Framework\Constraint\IsNull;
use PHPUnit\Framework\Constraint\IsReadable;
use PHPUnit\Framework\Constraint\IsTrue;
use PHPUnit\Framework\Constraint\IsType;
use PHPUnit\Framework\Constraint\IsWritable;
use PHPUnit\Framework\Constraint\LessThan;
use PHPUnit\Framework\Constraint\LogicalAnd;
use PHPUnit\Framework\Constraint\LogicalNot;
use PHPUnit\Framework\Constraint\LogicalOr;
use PHPUnit\Framework\Constraint\LogicalXor;
use PHPUnit\Framework\Constraint\ObjectEquals;
use PHPUnit\Framework\Constraint\RegularExpression;
use PHPUnit\Framework\Constraint\StringContains;
use PHPUnit\Framework\Constraint\StringEndsWith;
use PHPUnit\Framework\Constraint\StringEqualsStringIgnoringLineEndings;
use PHPUnit\Framework\Constraint\StringEqualsStringIgnoringWhitespace;
use PHPUnit\Framework\Constraint\StringMatchesFormatDescription;
use PHPUnit\Framework\Constraint\StringStartsWith;
use PHPUnit\Framework\Constraint\TraversableContainsEqual;
use PHPUnit\Framework\Constraint\TraversableContainsIdentical;
use PHPUnit\Framework\Constraint\TraversableContainsOnly;
use PHPUnit\Framework\MockObject\Rule\AnyInvokedCount as AnyInvokedCountMatcher;
use PHPUnit\Framework\MockObject\Rule\InvokedAtLeastCount as InvokedAtLeastCountMatcher;
use PHPUnit\Framework\MockObject\Rule\InvokedAtLeastOnce as InvokedAtLeastOnceMatcher;
use PHPUnit\Framework\MockObject\Rule\InvokedAtMostCount as InvokedAtMostCountMatcher;
use PHPUnit\Framework\MockObject\Rule\InvokedCount as InvokedCountMatcher;
use PHPUnit\Framework\MockObject\Stub\Exception as ExceptionStub;
use PHPUnit\Util\Xml\XmlException;
use Throwable;

if (!function_exists('PHPUnit\Framework\assertArrayIsEqualToArrayOnlyConsideringListOfKeys')) {
    /**
     * Asserts that two arrays are equal while only considering a list of keys.
     *
     * @param array<mixed>              $expected
     * @param array<mixed>              $actual
     * @param non-empty-list<array-key> $keysToBeConsidered
     *
     * @throws Exception
     * @throws ExpectationFailedException
     *
     * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
     *
     * @see Assert::assertArrayIsEqualToArrayOnlyConsideringListOfKeys
     */
    function assertArrayIsEqualToArrayOnlyConsideringListOfKeys(array $expected, array $actual, array $keysToBeConsidered, string $message = ''): void
    {}
}

if (!function_exists('PHPUnit\Framework\assertArrayIsEqualToArrayIgnoringListOfKeys')) {
    /**
     * Asserts that two arrays are equal while ignoring a list of keys.
     *
     * @param array<mixed>              $expected
     * @param array<mixed>              $actual
     * @param non-empty-list<array-key> $keysToBeIgnored
     *
     * @throws Exception
     * @throws ExpectationFailedException
     *
     * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
     *
     * @see Assert::assertArrayIsEqualToArrayIgnoringListOfKeys
     */
    function assertArrayIsEqualToArrayIgnoringListOfKeys(array $expected, array $actual, array $keysToBeIgnored, string $message = ''): void
    {}
}

if (!function_exists('PHPUnit\Framework\assertArrayIsIdenticalToArrayOnlyConsideringListOfKeys')) {
    /**
     * Asserts that two arrays are identical while only considering a list of keys.
     *
     * @param array<mixed>              $expected
     * @param array<mixed>              $actual
     * @param non-empty-list<array-key> $keysToBeConsidered
     *
     * @throws Exception
     * @throws ExpectationFailedException
     *
     * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
     *
     * @see Assert::assertArrayIsIdenticalToArrayOnlyConsideringListOfKeys
     */
    function assertArrayIsIdenticalToArrayOnlyConsideringListOfKeys(array $expected, array $actual, array $keysToBeConsidered, string $message = ''): void
    {}
}

if (!function_exists('PHPUnit\Framework\assertArrayIsIdenticalToArrayIgnoringListOfKeys')) {
    /**
     * Asserts that two arrays are equal while ignoring a list of keys.
     *
     * @param array<mixed>              $expected
     * @param array<mixed>              $actual
     * @param non-empty-list<array-key> $keysToBeIgnored
     *
     * @throws Exception
     * @throws ExpectationFailedException
     *
     * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
     *
     * @see Assert::assertArrayIsIdenticalToArrayIgnoringListOfKeys
     */
    function assertArrayIsIdenticalToArrayIgnoringListOfKeys(array $expected, array $actual, array $keysToBeIgnored, string $message = ''): void
    {}
}

if (!function_exists('PHPUnit\Framework\assertArrayHasKey')) {
    /**
     * Asserts that an array has a specified key.
     *
     * @param array<mixed>|ArrayAccess<array-key, covariant mixed> $array
     *
     * @throws Exception
     * @throws ExpectationFailedException
     *
     * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
     *
     * @see Assert::assertArrayHasKey
     */
    function assertArrayHasKey(mixed $key, array|ArrayAccess $array, string $message = ''): void
    {}
}

if (!function_exists('PHPUnit\Framework\assertArrayNotHasKey')) {
    /**
     * Asserts that an array does not have a specified key.
     *
     * @param array<mixed>|ArrayAccess<array-key, covariant mixed> $array
     *
     * @throws Exception
     * @throws ExpectationFailedException
     *
     * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
     *
     * @see Assert::assertArrayNotHasKey
     */
    function assertArrayNotHasKey(mixed $key, array|ArrayAccess $array, string $message = ''): void
    {}
}

if (!function_exists('PHPUnit\Framework\assertIsList')) {
    /**
     * @phpstan-assert list<mixed> $array
     *
     * @throws ExpectationFailedException
     *
     * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
     *
     * @see Assert::assertIsList
     */
    function assertIsList(mixed $array, string $message = ''): void
    {}
}

if (!function_exists('PHPUnit\Framework\assertArraysAreIdentical')) {
    /**
     * Assert that two arrays are identical.
     *
     * The (key, value) relationship matters, the order of the (key, value) pairs in the array matters, and keys as well as values are compared strictly.
     *
     * @param array<mixed> $expected
     * @param array<mixed> $actual
     *
     * @throws ExpectationFailedException
     *
     * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
     *
     * @see Assert::assertArraysAreIdentical
     */
    function assertArraysAreIdentical(array $expected, array $actual, string $message = ''): void
    {}
}

if (!function_exists('PHPUnit\Framework\assertArraysAreIdenticalIgnoringOrder')) {
    /**
     * Assert that two arrays are identical while ignoring the order of their values.
     *
     * The (key, value) relationship matters, the order of the (key, value) pairs in the array does not matter, and keys as well as values are compared strictly.
     *
     * @param array<mixed> $expected
     * @param array<mixed> $actual
     *
     * @throws ExpectationFailedException
     *
     * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
     *
     * @see Assert::assertArraysAreIdenticalIgnoringOrder
     */
    function assertArraysAreIdenticalIgnoringOrder(array $expected, array $actual, string $message = ''): void
    {}
}

if (!function_exists('PHPUnit\Framework\assertArraysHaveIdenticalValues')) {
    /**
     * Assert that two arrays have identical values.
     *
     * The (key, value) relationship does not matter, the order of the (key, value) pairs in the array matters, and values are compared strictly.
     *
     * @param array<mixed> $expected
     * @param array<mixed> $actual
     *
     * @throws ExpectationFailedException
     *
     * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
     *
     * @see Assert::assertArraysHaveIdenticalValues
     */
    function assertArraysHaveIdenticalValues(array $expected, array $actual, string $message = ''): void
    {}
}

if (!function_exists('PHPUnit\Framework\assertArraysHaveIdenticalValuesIgnoringOrder')) {
    /**
     * Assert that two arrays have identical values while ignoring the order of these values.
     *
     * The (key, value) relationship does not matter, the order of the (key, value) pairs in the array does not matter, and values are compared strictly.
     *
     * @param array<mixed> $expected
     * @param array<mixed> $actual
     *
     * @throws ExpectationFailedException
     *
     * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
     *
     * @see Assert::assertArraysHaveIdenticalValuesIgnoringOrder
     */
    function assertArraysHaveIdenticalValuesIgnoringOrder(array $expected, array $actual, string $message = ''): void
    {}
}

if (!function_exists('PHPUnit\Framework\assertArraysAreEqual')) {
    /**
     * Assert that two arrays are equal.
     *
     * The (key, value) relationship matters, the order of the (key, value) pairs in the array matters, and keys as well as values are compared loosely.
     *
     * @param array<mixed> $expected
     * @param array<mixed> $actual
     *
     * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
     *
     * @see Assert::assertArraysAreEqual
     */
    function assertArraysAreEqual(array $expected, array $actual, string $message = ''): void
    {}
}

if (!function_exists('PHPUnit\Framework\assertArraysAreEqualIgnoringOrder')) {
    /**
     * Assert that two arrays are equal while ignoring the order of their values.
     *
     * The (key, value) relationship matters, the order of the (key, value) pairs in the array does not matter, and keys as well as values are compared loosely.
     *
     * @param array<mixed> $expected
     * @param array<mixed> $actual
     *
     * @throws ExpectationFailedException
     *
     * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
     *
     * @see Assert::assertArraysAreEqualIgnoringOrder
     */
    function assertArraysAreEqualIgnoringOrder(array $expected, array $actual, string $message = ''): void
    {}
}

if (!function_exists('PHPUnit\Framework\assertArraysHaveEqualValues')) {
    /**
     * Assert that two arrays have equal values.
     *
     * The (key, value) relationship does not matter, the order of the (key, value) pairs in the array matters, and values are compared loosely.
     *
     * @param array<mixed> $expected
     * @param array<mixed> $actual
     *
     * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
     *
     * @see Assert::assertArraysHaveEqualValues
     */
    function assertArraysHaveEqualValues(array $expected, array $actual, string $message = ''): void
    {}
}

if (!function_exists('PHPUnit\Framework\assertArraysHaveEqualValuesIgnoringOrder')) {
    /**
     * Assert that two arrays have equal values while ignoring the order of these values.
     *
     * The (key, value) relationship does not matter, the order of the (key, value) pairs in the array does not matter, and values are compared loosely.
     *
     * @param array<mixed> $expected
     * @param array<mixed> $actual
     *
     * @throws ExpectationFailedException
     *
     * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
     *
     * @see Assert::assertArraysHaveEqualValuesIgnoringOrder
     */
    function assertArraysHaveEqualValuesIgnoringOrder(array $expected, array $actual, string $message = ''): void
    {}
}

if (!function_exists('PHPUnit\Framework\assertContains')) {
    /**
     * Asserts that a haystack contains a needle.
     *
     * @param iterable<mixed> $haystack
     *
     * @throws Exception
     * @throws ExpectationFailedException
     *
     * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
     *
     * @see Assert::assertContains
     */
    function assertContains(mixed $needle, iterable $haystack, string $message = ''): void
    {}
}

if (!function_exists('PHPUnit\Framework\assertContainsEquals')) {
    /**
     * @param iterable<mixed> $haystack
     *
     * @throws ExpectationFailedException
     *
     * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
     *
     * @see Assert::assertContainsEquals
     */
    function assertContainsEquals(mixed $needle, iterable $haystack, string $message = ''): void
    {}
}

if (!function_exists('PHPUnit\Framework\assertNotContains')) {
    /**
     * Asserts that a haystack does not contain a needle.
     *
     * @param iterable<mixed> $haystack
     *
     * @throws Exception
     * @throws ExpectationFailedException
     *
     * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
     *
     * @see Assert::assertNotContains
     */
    function assertNotContains(mixed $needle, iterable $haystack, string $message = ''): void
    {}
}

if (!function_exists('PHPUnit\Framework\assertNotContainsEquals')) {
    /**
     * @param iterable<mixed> $haystack
     *
     * @throws ExpectationFailedException
     *
     * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
     *
     * @see Assert::assertNotContainsEquals
     */
    function assertNotContainsEquals(mixed $needle, iterable $haystack, string $message = ''): void
    {}
}

if (!function_exists('PHPUnit\Framework\assertContainsOnlyArray')) {
    /**
     * Asserts that a haystack contains only values of type array.
     *
     * @phpstan-assert iterable<array<mixed>> $haystack
     *
     * @param iterable<mixed> $haystack
     *
     * @throws ExpectationFailedException
     *
     * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
     *
     * @see Assert::assertContainsOnlyArray
     */
    function assertContainsOnlyArray(iterable $haystack, string $message = ''): void
    {}
}

if (!function_exists('PHPUnit\Framework\assertContainsOnlyBool')) {
    /**
     * Asserts that a haystack contains only values of type bool.
     *
     * @phpstan-assert iterable<bool> $haystack
     *
     * @param iterable<mixed> $haystack
     *
     * @throws ExpectationFailedException
     *
     * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
     *
     * @see Assert::assertContainsOnlyBool
     */
    function assertContainsOnlyBool(iterable $haystack, string $message = ''): void
    {}
}

if (!function_exists('PHPUnit\Framework\assertContainsOnlyCallable')) {
    /**
     * Asserts that a haystack contains only values of type callable.
     *
     * @phpstan-assert iterable<callable> $haystack
     *
     * @param iterable<mixed> $haystack
     *
     * @throws ExpectationFailedException
     *
     * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
     *
     * @see Assert::assertContainsOnlyCallable
     */
    function assertContainsOnlyCallable(iterable $haystack, string $message = ''): void
    {}
}

if (!function_exists('PHPUnit\Framework\assertContainsOnlyFloat')) {
    /**
     * Asserts that a haystack contains only values of type float.
     *
     * @phpstan-assert iterable<float> $haystack
     *
     * @param iterable<mixed> $haystack
     *
     * @throws ExpectationFailedException
     *
     * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
     *
     * @see Assert::assertContainsOnlyFloat
     */
    function assertContainsOnlyFloat(iterable $haystack, string $message = ''): void
    {}
}

if (!function_exists('PHPUnit\Framework\assertContainsOnlyInt')) {
    /**
     * Asserts that a haystack contains only values of type int.
     *
     * @phpstan-assert iterable<int> $haystack
     *
     * @param iterable<mixed> $haystack
     *
     * @throws ExpectationFailedException
     *
     * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
     *
     * @see Assert::assertContainsOnlyInt
     */
    function assertContainsOnlyInt(iterable $haystack, string $message = ''): void
    {}
}

if (!function_exists('PHPUnit\Framework\assertContainsOnlyIterable')) {
    /**
     * Asserts that a haystack contains only values of type iterable.
     *
     * @phpstan-assert iterable<iterable<mixed>> $haystack
     *
     * @param iterable<mixed> $haystack
     *
     * @throws ExpectationFailedException
     *
     * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
     *
     * @see Assert::assertContainsOnlyIterable
     */
    function assertContainsOnlyIterable(iterable $haystack, string $message = ''): void
    {}
}

if (!function_exists('PHPUnit\Framework\assertContainsOnlyNull')) {
    /**
     * Asserts that a haystack contains only values of type null.
     *
     * @phpstan-assert iterable<null> $haystack
     *
     * @param iterable<mixed> $haystack
     *
     * @throws ExpectationFailedException
     *
     * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
     *
     * @see Assert::assertContainsOnlyNull
     */
    function assertContainsOnlyNull(iterable $haystack, string $message = ''): void
    {}
}

if (!function_exists('PHPUnit\Framework\assertContainsOnlyNumeric')) {
    /**
     * Asserts that a haystack contains only values of type numeric.
     *
     * @phpstan-assert iterable<numeric> $haystack
     *
     * @param iterable<mixed> $haystack
     *
     * @throws ExpectationFailedException
     *
     * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
     *
     * @see Assert::assertContainsOnlyNumeric
     */
    function assertContainsOnlyNumeric(iterable $haystack, string $message = ''): void
    {}
}

if (!function_exists('PHPUnit\Framework\assertContainsOnlyObject')) {
    /**
     * Asserts that a haystack contains only values of type object.
     *
     * @phpstan-assert iterable<object> $haystack
     *
     * @param iterable<mixed> $haystack
     *
     * @throws ExpectationFailedException
     *
     * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
     *
     * @see Assert::assertContainsOnlyObject
     */
    function assertContainsOnlyObject(iterable $haystack, string $message = ''): void
    {}
}

if (!function_exists('PHPUnit\Framework\assertContainsOnlyResource')) {
    /**
     * Asserts that a haystack contains only values of type resource.
     *
     * @phpstan-assert iterable<resource> $haystack
     *
     * @param iterable<mixed> $haystack
     *
     * @throws ExpectationFailedException
     *
     * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
     *
     * @see Assert::assertContainsOnlyResource
     */
    function assertContainsOnlyResource(iterable $haystack, string $message = ''): void
    {}
}

if (!function_exists('PHPUnit\Framework\assertContainsOnlyClosedResource')) {
    /**
     * Asserts that a haystack contains only values of type closed resource.
     *
     * @phpstan-assert iterable<resource> $haystack
     *
     * @param iterable<mixed> $haystack
     *
     * @throws ExpectationFailedException
     *
     * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
     *
     * @see Assert::assertContainsOnlyClosedResource
     */
    function assertContainsOnlyClosedResource(iterable $haystack, string $message = ''): void
    {}
}

if (!function_exists('PHPUnit\Framework\assertContainsOnlyScalar')) {
    /**
     * Asserts that a haystack contains only values of type scalar.
     *
     * @phpstan-assert iterable<scalar> $haystack
     *
     * @param iterable<mixed> $haystack
     *
     * @throws ExpectationFailedException
     *
     * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
     *
     * @see Assert::assertContainsOnlyScalar
     */
    function assertContainsOnlyScalar(iterable $haystack, string $message = ''): void
    {}
}

if (!function_exists('PHPUnit\Framework\assertContainsOnlyString')) {
    /**
     * Asserts that a haystack contains only values of type string.
     *
     * @phpstan-assert iterable<string> $haystack
     *
     * @param iterable<mixed> $haystack
     *
     * @throws ExpectationFailedException
     *
     * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
     *
     * @see Assert::assertContainsOnlyString
     */
    function assertContainsOnlyString(iterable $haystack, string $message = ''): void
    {}
}

if (!function_exists('PHPUnit\Framework\assertContainsOnlyInstancesOf')) {
    /**
     * Asserts that a haystack contains only instances of a specified interface or class name.
     *
     * @template T of object
     *
     * @phpstan-assert iterable<T> $haystack
     *
     * @param class-string<T> $className
     * @param iterable<mixed> $haystack
     *
     * @throws Exception
     * @throws ExpectationFailedException
     *
     * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
     *
     * @see Assert::assertContainsOnlyInstancesOf
     */
    function assertContainsOnlyInstancesOf(string $className, iterable $haystack, string $message = ''): void
    {}
}

if (!function_exists('PHPUnit\Framework\assertContainsNotOnlyArray')) {
    /**
     * Asserts that a haystack does not contain only values of type array.
     *
     * @param iterable<mixed> $haystack
     *
     * @throws ExpectationFailedException
     *
     * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
     *
     * @see Assert::assertContainsNotOnlyArray
     */
    function assertContainsNotOnlyArray(iterable $haystack, string $message = ''): void
    {}
}

if (!function_exists('PHPUnit\Framework\assertContainsNotOnlyBool')) {
    /**
     * Asserts that a haystack does not contain only values of type bool.
     *
     * @param iterable<mixed> $haystack
     *
     * @throws ExpectationFailedException
     *
     * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
     *
     * @see Assert::assertContainsNotOnlyBool
     */
    function assertContainsNotOnlyBool(iterable $haystack, string $message = ''): void
    {}
}

if (!function_exists('PHPUnit\Framework\assertContainsNotOnlyCallable')) {
    /**
     * Asserts that a haystack does not contain only values of type callable.
     *
     * @param iterable<mixed> $haystack
     *
     * @throws ExpectationFailedException
     *
     * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
     *
     * @see Assert::assertContainsNotOnlyCallable
     */
    function assertContainsNotOnlyCallable(iterable $haystack, string $message = ''): void
    {}
}

if (!function_exists('PHPUnit\Framework\assertContainsNotOnlyFloat')) {
    /**
     * Asserts that a haystack does not contain only values of type float.
     *
     * @param iterable<mixed> $haystack
     *
     * @throws ExpectationFailedException
     *
     * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
     *
     * @see Assert::assertContainsNotOnlyFloat
     */
    function assertContainsNotOnlyFloat(iterable $haystack, string $message = ''): void
    {}
}

if (!function_exists('PHPUnit\Framework\assertContainsNotOnlyInt')) {
    /**
     * Asserts that a haystack does not contain only values of type int.
     *
     * @param iterable<mixed> $haystack
     *
     * @throws ExpectationFailedException
     *
     * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
     *
     * @see Assert::assertContainsNotOnlyInt
     */
    function assertContainsNotOnlyInt(iterable $haystack, string $message = ''): void
    {}
}

if (!function_exists('PHPUnit\Framework\assertContainsNotOnlyIterable')) {
    /**
     * Asserts that a haystack does not contain only values of type iterable.
     *
     * @param iterable<mixed> $haystack
     *
     * @throws ExpectationFailedException
     *
     * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
     *
     * @see Assert::assertContainsNotOnlyIterable
     */
    function assertContainsNotOnlyIterable(iterable $haystack, string $message = ''): void
    {}
}

if (!function_exists('PHPUnit\Framework\assertContainsNotOnlyNull')) {
    /**
     * Asserts that a haystack does not contain only values of type null.
     *
     * @param iterable<mixed> $haystack
     *
     * @throws ExpectationFailedException
     *
     * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
     *
     * @see Assert::assertContainsNotOnlyNull
     */
    function assertContainsNotOnlyNull(iterable $haystack, string $message = ''): void
    {}
}

if (!function_exists('PHPUnit\Framework\assertContainsNotOnlyNumeric')) {
    /**
     * Asserts that a haystack does not contain only values of type numeric.
     *
     * @param iterable<mixed> $haystack
     *
     * @throws ExpectationFailedException
     *
     * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
     *
     * @see Assert::assertContainsNotOnlyNumeric
     */
    function assertContainsNotOnlyNumeric(iterable $haystack, string $message = ''): void
    {}
}

if (!function_exists('PHPUnit\Framework\assertContainsNotOnlyObject')) {
    /**
     * Asserts that a haystack does not contain only values of type object.
     *
     * @param iterable<mixed> $haystack
     *
     * @throws ExpectationFailedException
     *
     * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
     *
     * @see Assert::assertContainsNotOnlyObject
     */
    function assertContainsNotOnlyObject(iterable $haystack, string $message = ''): void
    {}
}

if (!function_exists('PHPUnit\Framework\assertContainsNotOnlyResource')) {
    /**
     * Asserts that a haystack does not contain only values of type resource.
     *
     * @param iterable<mixed> $haystack
     *
     * @throws ExpectationFailedException
     *
     * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
     *
     * @see Assert::assertContainsNotOnlyResource
     */
    function assertContainsNotOnlyResource(iterable $haystack, string $message = ''): void
    {}
}

if (!function_exists('PHPUnit\Framework\assertContainsNotOnlyClosedResource')) {
    /**
     * Asserts that a haystack does not contain only values of type closed resource.
     *
     * @param iterable<mixed> $haystack
     *
     * @throws ExpectationFailedException
     *
     * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
     *
     * @see Assert::assertContainsNotOnlyClosedResource
     */
    function assertContainsNotOnlyClosedResource(iterable $haystack, string $message = ''): void
    {}
}

if (!function_exists('PHPUnit\Framework\assertContainsNotOnlyScalar')) {
    /**
     * Asserts that a haystack does not contain only values of type scalar.
     *
     * @param iterable<mixed> $haystack
     *
     * @throws ExpectationFailedException
     *
     * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
     *
     * @see Assert::assertContainsNotOnlyScalar
     */
    function assertContainsNotOnlyScalar(iterable $haystack, string $message = ''): void
    {}
}

if (!function_exists('PHPUnit\Framework\assertContainsNotOnlyString')) {
    /**
     * Asserts that a haystack does not contain only values of type string.
     *
     * @param iterable<mixed> $haystack
     *
     * @throws ExpectationFailedException
     *
     * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
     *
     * @see Assert::assertContainsNotOnlyString
     */
    function assertContainsNotOnlyString(iterable $haystack, string $message = ''): void
    {}
}

if (!function_exists('PHPUnit\Framework\assertContainsNotOnlyInstancesOf')) {
    /**
     * Asserts that a haystack does not contain only instances of a specified interface or class name.
     *
     * @param class-string    $className
     * @param iterable<mixed> $haystack
     *
     * @throws Exception
     * @throws ExpectationFailedException
     *
     * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
     *
     * @see Assert::assertContainsNotOnlyInstancesOf
     */
    function assertContainsNotOnlyInstancesOf(string $className, iterable $haystack, string $message = ''): void
    {}
}

if (!function_exists('PHPUnit\Framework\assertCount')) {
    /**
     * Asserts the number of elements of an array, Countable or Traversable.
     *
     * @param Countable|iterable<mixed> $haystack
     *
     * @throws Exception
     * @throws ExpectationFailedException
     * @throws GeneratorNotSupportedException
     *
     * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
     *
     * @see Assert::assertCount
     */
    function assertCount(int $expectedCount, Countable|iterable $haystack, string $message = ''): void
    {}
}

if (!function_exists('PHPUnit\Framework\assertNotCount')) {
    /**
     * Asserts the number of elements of an array, Countable or Traversable.
     *
     * @param Countable|iterable<mixed> $haystack
     *
     * @throws Exception
     * @throws ExpectationFailedException
     * @throws GeneratorNotSupportedException
     *
     * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
     *
     * @see Assert::assertNotCount
     */
    function assertNotCount(int $expectedCount, Countable|iterable $haystack, string $message = ''): void
    {}
}

if (!function_exists('PHPUnit\Framework\assertEquals')) {
    /**
     * Asserts that two variables are equal.
     *
     * Comparison is performed using the == operator (loose comparison) and may be performed by a type-specific comparator which may apply type coercion.
     *
     * @throws ExpectationFailedException
     *
     * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
     *
     * @see Assert::assertEquals
     */
    function assertEquals(mixed $expected, mixed $actual, string $message = ''): void
    {}
}

if (!function_exists('PHPUnit\Framework\assertEqualsCanonicalizing')) {
    /**
     * Asserts that two variables are equal (canonicalizing).
     *
     * Comparison is performed using the == operator (loose comparison) and may be performed by a type-specific comparator which may apply type coercion.
     *
     * @throws ExpectationFailedException
     *
     * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
     *
     * @see Assert::assertEqualsCanonicalizing
     */
    function assertEqualsCanonicalizing(mixed $expected, mixed $actual, string $message = ''): void
    {}
}

if (!function_exists('PHPUnit\Framework\assertEqualsIgnoringCase')) {
    /**
     * Asserts that two variables are equal (ignoring case).
     *
     * Comparison is performed using the == operator (loose comparison) and may be performed by a type-specific comparator which may apply type coercion.
     *
     * @throws ExpectationFailedException
     *
     * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
     *
     * @see Assert::assertEqualsIgnoringCase
     */
    function assertEqualsIgnoringCase(mixed $expected, mixed $actual, string $message = ''): void
    {}
}

if (!function_exists('PHPUnit\Framework\assertEqualsWithDelta')) {
    /**
     * Asserts that two variables are equal (with delta).
     *
     * Comparison is performed using the == operator (loose comparison) and may be performed by a type-specific comparator which may apply type coercion.
     *
     * @throws ExpectationFailedException
     *
     * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
     *
     * @see Assert::assertEqualsWithDelta
     */
    function assertEqualsWithDelta(mixed $expected, mixed $actual, float $delta, string $message = ''): void
    {}
}

if (!function_exists('PHPUnit\Framework\assertNotEquals')) {
    /**
     * Asserts that two variables are not equal.
     *
     * Comparison is performed using the == operator (loose comparison) and may be performed by a type-specific comparator which may apply type coercion.
     *
     * @throws ExpectationFailedException
     *
     * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
     *
     * @see Assert::assertNotEquals
     */
    function assertNotEquals(mixed $expected, mixed $actual, string $message = ''): void
    {}
}

if (!function_exists('PHPUnit\Framework\assertNotEqualsCanonicalizing')) {
    /**
     * Asserts that two variables are not equal (canonicalizing).
     *
     * Comparison is performed using the == operator (loose comparison) and may be performed by a type-specific comparator which may apply type coercion.
     *
     * @throws ExpectationFailedException
     *
     * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
     *
     * @see Assert::assertNotEqualsCanonicalizing
     */
    function assertNotEqualsCanonicalizing(mixed $expected, mixed $actual, string $message = ''): void
    {}
}

if (!function_exists('PHPUnit\Framework\assertNotEqualsIgnoringCase')) {
    /**
     * Asserts that two variables are not equal (ignoring case).
     *
     * Comparison is performed using the == operator (loose comparison) and may be performed by a type-specific comparator which may apply type coercion.
     *
     * @throws ExpectationFailedException
     *
     * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
     *
     * @see Assert::assertNotEqualsIgnoringCase
     */
    function assertNotEqualsIgnoringCase(mixed $expected, mixed $actual, string $message = ''): void
    {}
}

if (!function_exists('PHPUnit\Framework\assertNotEqualsWithDelta')) {
    /**
     * Asserts that two variables are not equal (with delta).
     *
     * Comparison is performed using the == operator (loose comparison) and may be performed by a type-specific comparator which may apply type coercion.
     *
     * @throws ExpectationFailedException
     *
     * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
     *
     * @see Assert::assertNotEqualsWithDelta
     */
    function assertNotEqualsWithDelta(mixed $expected, mixed $actual, float $delta, string $message = ''): void
    {}
}

if (!function_exists('PHPUnit\Framework\assertObjectEquals')) {
    /**
     * @throws ExpectationFailedException
     *
     * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
     *
     * @see Assert::assertObjectEquals
     */
    function assertObjectEquals(object $expected, object $actual, string $method = 'equals', string $message = ''): void
    {}
}

if (!function_exists('PHPUnit\Framework\assertObjectNotEquals')) {
    /**
     * @throws ExpectationFailedException
     *
     * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
     *
     * @see Assert::assertObjectNotEquals
     */
    function assertObjectNotEquals(object $expected, object $actual, string $method = 'equals', string $message = ''): void
    {}
}

if (!function_exists('PHPUnit\Framework\assertEmpty')) {
    /**
     * Asserts that a variable is empty.
     *
     * @throws ExpectationFailedException
     * @throws GeneratorNotSupportedException
     *
     * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
     *
     * @see Assert::assertEmpty
     */
    function assertEmpty(mixed $actual, string $message = ''): void
    {}
}

if (!function_exists('PHPUnit\Framework\assertNotEmpty')) {
    /**
     * Asserts that a variable is not empty.
     *
     * @throws ExpectationFailedException
     * @throws GeneratorNotSupportedException
     *
     * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
     *
     * @see Assert::assertNotEmpty
     */
    function assertNotEmpty(mixed $actual, string $message = ''): void
    {}
}

if (!function_exists('PHPUnit\Framework\assertGreaterThan')) {
    /**
     * Asserts that a value is greater than another value.
     *
     * @throws ExpectationFailedException
     *
     * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
     *
     * @see Assert::assertGreaterThan
     */
    function assertGreaterThan(mixed $minimum, mixed $actual, string $message = ''): void
    {}
}

if (!function_exists('PHPUnit\Framework\assertGreaterThanOrEqual')) {
    /**
     * Asserts that a value is greater than or equal to another value.
     *
     * @throws ExpectationFailedException
     *
     * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
     *
     * @see Assert::assertGreaterThanOrEqual
     */
    function assertGreaterThanOrEqual(mixed $minimum, mixed $actual, string $message = ''): void
    {}
}

if (!function_exists('PHPUnit\Framework\assertLessThan')) {
    /**
     * Asserts that a value is smaller than another value.
     *
     * @throws ExpectationFailedException
     *
     * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
     *
     * @see Assert::assertLessThan
     */
    function assertLessThan(mixed $maximum, mixed $actual, string $message = ''): void
    {}
}

if (!function_exists('PHPUnit\Framework\assertLessThanOrEqual')) {
    /**
     * Asserts that a value is smaller than or equal to another value.
     *
     * @throws ExpectationFailedException
     *
     * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
     *
     * @see Assert::assertLessThanOrEqual
     */
    function assertLessThanOrEqual(mixed $maximum, mixed $actual, string $message = ''): void
    {}
}

if (!function_exists('PHPUnit\Framework\assertFileEquals')) {
    /**
     * Asserts that the contents of one file is equal to the contents of another
     * file.
     *
     * @throws ExpectationFailedException
     *
     * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
     *
     * @see Assert::assertFileEquals
     */
    function assertFileEquals(string $expected, string $actual, string $message = ''): void
    {}
}

if (!function_exists('PHPUnit\Framework\assertFileEqualsCanonicalizing')) {
    /**
     * Asserts that the contents of one file is equal to the contents of another
     * file (canonicalizing).
     *
     * @throws ExpectationFailedException
     *
     * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
     *
     * @see Assert::assertFileEqualsCanonicalizing
     */
    function assertFileEqualsCanonicalizing(string $expected, string $actual, string $message = ''): void
    {}
}

if (!function_exists('PHPUnit\Framework\assertFileEqualsIgnoringCase')) {
    /**
     * Asserts that the contents of one file is equal to the contents of another
     * file (ignoring case).
     *
     * @throws ExpectationFailedException
     *
     * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
     *
     * @see Assert::assertFileEqualsIgnoringCase
     */
    function assertFileEqualsIgnoringCase(string $expected, string $actual, string $message = ''): void
    {}
}

if (!function_exists('PHPUnit\Framework\assertFileEqualsFileIgnoringWhitespace')) {
    /**
     * Asserts that the contents of one file is equal to the contents of another
     * file (ignoring whitespace).
     *
     * @throws ExpectationFailedException
     *
     * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
     *
     * @see Assert::assertFileEqualsFileIgnoringWhitespace
     */
    function assertFileEqualsFileIgnoringWhitespace(string $expected, string $actual, string $message = ''): void
    {}
}

if (!function_exists('PHPUnit\Framework\assertFileNotEquals')) {
    /**
     * Asserts that the contents of one file is not equal to the contents of
     * another file.
     *
     * @throws ExpectationFailedException
     *
     * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
     *
     * @see Assert::assertFileNotEquals
     */
    function assertFileNotEquals(string $expected, string $actual, string $message = ''): void
    {}
}

if (!function_exists('PHPUnit\Framework\assertFileNotEqualsCanonicalizing')) {
    /**
     * Asserts that the contents of one file is not equal to the contents of another
     * file (canonicalizing).
     *
     * @throws ExpectationFailedException
     *
     * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
     *
     * @see Assert::assertFileNotEqualsCanonicalizing
     */
    function assertFileNotEqualsCanonicalizing(string $expected, string $actual, string $message = ''): void
    {}
}

if (!function_exists('PHPUnit\Framework\assertFileNotEqualsIgnoringCase')) {
    /**
     * Asserts that the contents of one file is not equal to the contents of another
     * file (ignoring case).
     *
     * @throws ExpectationFailedException
     *
     * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
     *
     * @see Assert::assertFileNotEqualsIgnoringCase
     */
    function assertFileNotEqualsIgnoringCase(string $expected, string $actual, string $message = ''): void
    {}
}

if (!function_exists('PHPUnit\Framework\assertFileNotEqualsFileIgnoringWhitespace')) {
    /**
     * Asserts that the contents of one file is not equal to the contents of another
     * file (ignoring whitespace).
     *
     * @throws ExpectationFailedException
     *
     * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
     *
     * @see Assert::assertFileNotEqualsFileIgnoringWhitespace
     */
    function assertFileNotEqualsFileIgnoringWhitespace(string $expected, string $actual, string $message = ''): void
    {}
}

if (!function_exists('PHPUnit\Framework\assertStringEqualsFile')) {
    /**
     * Asserts that the contents of a string is equal
     * to the contents of a file.
     *
     * @throws ExpectationFailedException
     *
     * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
     *
     * @see Assert::assertStringEqualsFile
     */
    function assertStringEqualsFile(string $expectedFile, string $actualString, string $message = ''): void
    {}
}

if (!function_exists('PHPUnit\Framework\assertStringEqualsFileCanonicalizing')) {
    /**
     * Asserts that the contents of a string is equal
     * to the contents of a file (canonicalizing).
     *
     * @throws ExpectationFailedException
     *
     * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
     *
     * @see Assert::assertStringEqualsFileCanonicalizing
     */
    function assertStringEqualsFileCanonicalizing(string $expectedFile, string $actualString, string $message = ''): void
    {}
}

if (!function_exists('PHPUnit\Framework\assertStringEqualsFileIgnoringCase')) {
    /**
     * Asserts that the contents of a string is equal
     * to the contents of a file (ignoring case).
     *
     * @throws ExpectationFailedException
     *
     * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
     *
     * @see Assert::assertStringEqualsFileIgnoringCase
     */
    function assertStringEqualsFileIgnoringCase(string $expectedFile, string $actualString, string $message = ''): void
    {}
}

if (!function_exists('PHPUnit\Framework\assertStringNotEqualsFile')) {
    /**
     * Asserts that the contents of a string is not equal
     * to the contents of a file.
     *
     * @throws ExpectationFailedException
     *
     * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
     *
     * @see Assert::assertStringNotEqualsFile
     */
    function assertStringNotEqualsFile(string $expectedFile, string $actualString, string $message = ''): void
    {}
}

if (!function_exists('PHPUnit\Framework\assertStringNotEqualsFileCanonicalizing')) {
    /**
     * Asserts that the contents of a string is not equal
     * to the contents of a file (canonicalizing).
     *
     * @throws ExpectationFailedException
     *
     * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
     *
     * @see Assert::assertStringNotEqualsFileCanonicalizing
     */
    function assertStringNotEqualsFileCanonicalizing(string $expectedFile, string $actualString, string $message = ''): void
    {}
}

if (!function_exists('PHPUnit\Framework\assertStringNotEqualsFileIgnoringCase')) {
    /**
     * Asserts that the contents of a string is not equal
     * to the contents of a file (ignoring case).
     *
     * @throws ExpectationFailedException
     *
     * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
     *
     * @see Assert::assertStringNotEqualsFileIgnoringCase
     */
    function assertStringNotEqualsFileIgnoringCase(string $expectedFile, string $actualString, string $message = ''): void
    {}
}

if (!function_exists('PHPUnit\Framework\assertStringEqualsFileIgnoringWhitespace')) {
    /**
     * Asserts that the contents of a string is equal
     * to the contents of a file (ignoring whitespace).
     *
     * @throws ExpectationFailedException
     *
     * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
     *
     * @see Assert::assertStringEqualsFileIgnoringWhitespace
     */
    function assertStringEqualsFileIgnoringWhitespace(string $expectedFile, string $actualString, string $message = ''): void
    {}
}

if (!function_exists('PHPUnit\Framework\assertStringNotEqualsFileIgnoringWhitespace')) {
    /**
     * Asserts that the contents of a string is not equal
     * to the contents of a file (ignoring whitespace).
     *
     * @throws ExpectationFailedException
     *
     * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
     *
     * @see Assert::assertStringNotEqualsFileIgnoringWhitespace
     */
    function assertStringNotEqualsFileIgnoringWhitespace(string $expectedFile, string $actualString, string $message = ''): void
    {}
}

if (!function_exists('PHPUnit\Framework\assertIsReadable')) {
    /**
     * Asserts that a file/dir is readable.
     *
     * @throws ExpectationFailedException
     *
     * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
     *
     * @see Assert::assertIsReadable
     */
    function assertIsReadable(string $filename, string $message = ''): void
    {}
}

if (!function_exists('PHPUnit\Framework\assertIsNotReadable')) {
    /**
     * Asserts that a file/dir exists and is not readable.
     *
     * @throws ExpectationFailedException
     *
     * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
     *
     * @see Assert::assertIsNotReadable
     */
    function assertIsNotReadable(string $filename, string $message = ''): void
    {}
}

if (!function_exists('PHPUnit\Framework\assertIsWritable')) {
    /**
     * Asserts that a file/dir exists and is writable.
     *
     * @throws ExpectationFailedException
     *
     * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
     *
     * @see Assert::assertIsWritable
     */
    function assertIsWritable(string $filename, string $message = ''): void
    {}
}

if (!function_exists('PHPUnit\Framework\assertIsNotWritable')) {
    /**
     * Asserts that a file/dir exists and is not writable.
     *
     * @throws ExpectationFailedException
     *
     * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
     *
     * @see Assert::assertIsNotWritable
     */
    function assertIsNotWritable(string $filename, string $message = ''): void
    {}
}

if (!function_exists('PHPUnit\Framework\assertDirectoryExists')) {
    /**
     * Asserts that a directory exists.
     *
     * @throws ExpectationFailedException
     *
     * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
     *
     * @see Assert::assertDirectoryExists
     */
    function assertDirectoryExists(string $directory, string $message = ''): void
    {}
}

if (!function_exists('PHPUnit\Framework\assertDirectoryDoesNotExist')) {
    /**
     * Asserts that a directory does not exist.
     *
     * @throws ExpectationFailedException
     *
     * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
     *
     * @see Assert::assertDirectoryDoesNotExist
     */
    function assertDirectoryDoesNotExist(string $directory, string $message = ''): void
    {}
}

if (!function_exists('PHPUnit\Framework\assertDirectoryIsReadable')) {
    /**
     * Asserts that a directory exists and is readable.
     *
     * @throws ExpectationFailedException
     *
     * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
     *
     * @see Assert::assertDirectoryIsReadable
     */
    function assertDirectoryIsReadable(string $directory, string $message = ''): void
    {}
}

if (!function_exists('PHPUnit\Framework\assertDirectoryIsNotReadable')) {
    /**
     * Asserts that a directory exists and is not readable.
     *
     * @throws ExpectationFailedException
     *
     * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
     *
     * @see Assert::assertDirectoryIsNotReadable
     */
    function assertDirectoryIsNotReadable(string $directory, string $message = ''): void
    {}
}

if (!function_exists('PHPUnit\Framework\assertDirectoryIsWritable')) {
    /**
     * Asserts that a directory exists and is writable.
     *
     * @throws ExpectationFailedException
     *
     * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
     *
     * @see Assert::assertDirectoryIsWritable
     */
    function assertDirectoryIsWritable(string $directory, string $message = ''): void
    {}
}

if (!function_exists('PHPUnit\Framework\assertDirectoryIsNotWritable')) {
    /**
     * Asserts that a directory exists and is not writable.
     *
     * @throws ExpectationFailedException
     *
     * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
     *
     * @see Assert::assertDirectoryIsNotWritable
     */
    function assertDirectoryIsNotWritable(string $directory, string $message = ''): void
    {}
}

if (!function_exists('PHPUnit\Framework\assertFileExists')) {
    /**
     * Asserts that a file exists.
     *
     * @throws ExpectationFailedException
     *
     * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
     *
     * @see Assert::assertFileExists
     */
    function assertFileExists(string $filename, string $message = ''): void
    {}
}

if (!function_exists('PHPUnit\Framework\assertFileDoesNotExist')) {
    /**
     * Asserts that a file does not exist.
     *
     * @throws ExpectationFailedException
     *
     * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
     *
     * @see Assert::assertFileDoesNotExist
     */
    function assertFileDoesNotExist(string $filename, string $message = ''): void
    {}
}

if (!function_exists('PHPUnit\Framework\assertFileIsReadable')) {
    /**
     * Asserts that a file exists and is readable.
     *
     * @throws ExpectationFailedException
     *
     * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
     *
     * @see Assert::assertFileIsReadable
     */
    function assertFileIsReadable(string $file, string $message = ''): void
    {}
}

if (!function_exists('PHPUnit\Framework\assertFileIsNotReadable')) {
    /**
     * Asserts that a file exists and is not readable.
     *
     * @throws ExpectationFailedException
     *
     * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
     *
     * @see Assert::assertFileIsNotReadable
     */
    function assertFileIsNotReadable(string $file, string $message = ''): void
    {}
}

if (!function_exists('PHPUnit\Framework\assertFileIsWritable')) {
    /**
     * Asserts that a file exists and is writable.
     *
     * @throws ExpectationFailedException
     *
     * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
     *
     * @see Assert::assertFileIsWritable
     */
    function assertFileIsWritable(string $file, string $message = ''): void
    {}
}

if (!function_exists('PHPUnit\Framework\assertFileIsNotWritable')) {
    /**
     * Asserts that a file exists and is not writable.
     *
     * @throws ExpectationFailedException
     *
     * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
     *
     * @see Assert::assertFileIsNotWritable
     */
    function assertFileIsNotWritable(string $file, string $message = ''): void
    {}
}

if (!function_exists('PHPUnit\Framework\assertTrue')) {
    /**
     * Asserts that a condition is true.
     *
     * @throws ExpectationFailedException
     *
     * @phpstan-assert true $condition
     *
     * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
     *
     * @see Assert::assertTrue
     */
    function assertTrue(mixed $condition, string $message = ''): void
    {}
}

if (!function_exists('PHPUnit\Framework\assertNotTrue')) {
    /**
     * Asserts that a condition is not true.
     *
     * @throws ExpectationFailedException
     *
     * @phpstan-assert !true $condition
     *
     * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
     *
     * @see Assert::assertNotTrue
     */
    function assertNotTrue(mixed $condition, string $message = ''): void
    {}
}

if (!function_exists('PHPUnit\Framework\assertFalse')) {
    /**
     * Asserts that a condition is false.
     *
     * @throws ExpectationFailedException
     *
     * @phpstan-assert false $condition
     *
     * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
     *
     * @see Assert::assertFalse
     */
    function assertFalse(mixed $condition, string $message = ''): void
    {}
}

if (!function_exists('PHPUnit\Framework\assertNotFalse')) {
    /**
     * Asserts that a condition is not false.
     *
     * @throws ExpectationFailedException
     *
     * @phpstan-assert !false $condition
     *
     * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
     *
     * @see Assert::assertNotFalse
     */
    function assertNotFalse(mixed $condition, string $message = ''): void
    {}
}

if (!function_exists('PHPUnit\Framework\assertNull')) {
    /**
     * Asserts that a variable is null.
     *
     * @throws ExpectationFailedException
     *
     * @phpstan-assert null $actual
     *
     * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
     *
     * @see Assert::assertNull
     */
    function assertNull(mixed $actual, string $message = ''): void
    {}
}

if (!function_exists('PHPUnit\Framework\assertNotNull')) {
    /**
     * Asserts that a variable is not null.
     *
     * @throws ExpectationFailedException
     *
     * @phpstan-assert !null $actual
     *
     * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
     *
     * @see Assert::assertNotNull
     */
    function assertNotNull(mixed $actual, string $message = ''): void
    {}
}

if (!function_exists('PHPUnit\Framework\assertFinite')) {
    /**
     * Asserts that a variable is finite.
     *
     * @throws ExpectationFailedException
     *
     * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
     *
     * @see Assert::assertFinite
     */
    function assertFinite(mixed $actual, string $message = ''): void
    {}
}

if (!function_exists('PHPUnit\Framework\assertInfinite')) {
    /**
     * Asserts that a variable is infinite.
     *
     * @throws ExpectationFailedException
     *
     * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
     *
     * @see Assert::assertInfinite
     */
    function assertInfinite(mixed $actual, string $message = ''): void
    {}
}

if (!function_exists('PHPUnit\Framework\assertNan')) {
    /**
     * Asserts that a variable is nan.
     *
     * @throws ExpectationFailedException
     *
     * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
     *
     * @see Assert::assertNan
     */
    function assertNan(mixed $actual, string $message = ''): void
    {}
}

if (!function_exists('PHPUnit\Framework\assertObjectHasProperty')) {
    /**
     * Asserts that an object has a specified property.
     *
     * @throws ExpectationFailedException
     *
     * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
     *
     * @see Assert::assertObjectHasProperty
     */
    function assertObjectHasProperty(string $propertyName, object $object, string $message = ''): void
    {}
}

if (!function_exists('PHPUnit\Framework\assertObjectNotHasProperty')) {
    /**
     * Asserts that an object does not have a specified property.
     *
     * @throws ExpectationFailedException
     *
     * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
     *
     * @see Assert::assertObjectNotHasProperty
     */
    function assertObjectNotHasProperty(string $propertyName, object $object, string $message = ''): void
    {}
}

if (!function_exists('PHPUnit\Framework\assertSame')) {
    /**
     * Asserts that two variables have the same type and value.
     * Used on objects, it asserts that two variables reference
     * the same object.
     *
     * Comparison is performed using the === operator.
     *
     * @template ExpectedType
     *
     * @param ExpectedType $expected
     *
     * @throws ExpectationFailedException
     *
     * @phpstan-assert =ExpectedType $actual
     *
     * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
     *
     * @see Assert::assertSame
     */
    function assertSame(mixed $expected, mixed $actual, string $message = ''): void
    {}
}

if (!function_exists('PHPUnit\Framework\assertNotSame')) {
    /**
     * Asserts that two variables do not have the same type and value.
     * Used on objects, it asserts that two variables do not reference
     * the same object.
     *
     * Comparison is performed using the === operator.
     *
     * @throws ExpectationFailedException
     *
     * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
     *
     * @see Assert::assertNotSame
     */
    function assertNotSame(mixed $expected, mixed $actual, string $message = ''): void
    {}
}

if (!function_exists('PHPUnit\Framework\assertInstanceOf')) {
    /**
     * Asserts that a variable is of a given type.
     *
     * @template ExpectedType of object
     *
     * @param class-string<ExpectedType> $expected
     *
     * @throws Exception
     * @throws ExpectationFailedException
     * @throws UnknownClassOrInterfaceException
     *
     * @phpstan-assert =ExpectedType $actual
     *
     * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
     *
     * @see Assert::assertInstanceOf
     */
    function assertInstanceOf(string $expected, mixed $actual, string $message = ''): void
    {}
}

if (!function_exists('PHPUnit\Framework\assertNotInstanceOf')) {
    /**
     * Asserts that a variable is not of a given type.
     *
     * @template ExpectedType of object
     *
     * @param class-string<ExpectedType> $expected
     *
     * @throws Exception
     * @throws ExpectationFailedException
     *
     * @phpstan-assert !ExpectedType $actual
     *
     * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
     *
     * @see Assert::assertNotInstanceOf
     */
    function assertNotInstanceOf(string $expected, mixed $actual, string $message = ''): void
    {}
}

if (!function_exists('PHPUnit\Framework\assertIsArray')) {
    /**
     * Asserts that a variable is of type array.
     *
     * @throws Exception
     * @throws ExpectationFailedException
     *
     * @phpstan-assert array<mixed> $actual
     *
     * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
     *
     * @see Assert::assertIsArray
     */
    function assertIsArray(mixed $actual, string $message = ''): void
    {}
}

if (!function_exists('PHPUnit\Framework\assertIsBool')) {
    /**
     * Asserts that a variable is of type bool.
     *
     * @throws Exception
     * @throws ExpectationFailedException
     *
     * @phpstan-assert bool $actual
     *
     * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
     *
     * @see Assert::assertIsBool
     */
    function assertIsBool(mixed $actual, string $message = ''): void
    {}
}

if (!function_exists('PHPUnit\Framework\assertIsFloat')) {
    /**
     * Asserts that a variable is of type float.
     *
     * @throws Exception
     * @throws ExpectationFailedException
     *
     * @phpstan-assert float $actual
     *
     * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
     *
     * @see Assert::assertIsFloat
     */
    function assertIsFloat(mixed $actual, string $message = ''): void
    {}
}

if (!function_exists('PHPUnit\Framework\assertIsInt')) {
    /**
     * Asserts that a variable is of type int.
     *
     * @throws Exception
     * @throws ExpectationFailedException
     *
     * @phpstan-assert int $actual
     *
     * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
     *
     * @see Assert::assertIsInt
     */
    function assertIsInt(mixed $actual, string $message = ''): void
    {}
}

if (!function_exists('PHPUnit\Framework\assertIsNumeric')) {
    /**
     * Asserts that a variable is of type numeric.
     *
     * @throws Exception
     * @throws ExpectationFailedException
     *
     * @phpstan-assert numeric $actual
     *
     * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
     *
     * @see Assert::assertIsNumeric
     */
    function assertIsNumeric(mixed $actual, string $message = ''): void
    {}
}

if (!function_exists('PHPUnit\Framework\assertIsObject')) {
    /**
     * Asserts that a variable is of type object.
     *
     * @throws Exception
     * @throws ExpectationFailedException
     *
     * @phpstan-assert object $actual
     *
     * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
     *
     * @see Assert::assertIsObject
     */
    function assertIsObject(mixed $actual, string $message = ''): void
    {}
}

if (!function_exists('PHPUnit\Framework\assertIsResource')) {
    /**
     * Asserts that a variable is of type resource.
     *
     * @throws Exception
     * @throws ExpectationFailedException
     *
     * @phpstan-assert resource $actual
     *
     * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
     *
     * @see Assert::assertIsResource
     */
    function assertIsResource(mixed $actual, string $message = ''): void
    {}
}

if (!function_exists('PHPUnit\Framework\assertIsClosedResource')) {
    /**
     * Asserts that a variable is of type resource and is closed.
     *
     * @throws Exception
     * @throws ExpectationFailedException
     *
     * @phpstan-assert resource $actual
     *
     * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
     *
     * @see Assert::assertIsClosedResource
     */
    function assertIsClosedResource(mixed $actual, string $message = ''): void
    {}
}

if (!function_exists('PHPUnit\Framework\assertIsString')) {
    /**
     * Asserts that a variable is of type string.
     *
     * @throws Exception
     * @throws ExpectationFailedException
     *
     * @phpstan-assert string $actual
     *
     * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
     *
     * @see Assert::assertIsString
     */
    function assertIsString(mixed $actual, string $message = ''): void
    {}
}

if (!function_exists('PHPUnit\Framework\assertIsScalar')) {
    /**
     * Asserts that a variable is of type scalar.
     *
     * @throws Exception
     * @throws ExpectationFailedException
     *
     * @phpstan-assert scalar $actual
     *
     * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
     *
     * @see Assert::assertIsScalar
     */
    function assertIsScalar(mixed $actual, string $message = ''): void
    {}
}

if (!function_exists('PHPUnit\Framework\assertIsCallable')) {
    /**
     * Asserts that a variable is of type callable.
     *
     * @throws Exception
     * @throws ExpectationFailedException
     *
     * @phpstan-assert callable $actual
     *
     * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
     *
     * @see Assert::assertIsCallable
     */
    function assertIsCallable(mixed $actual, string $message = ''): void
    {}
}

if (!function_exists('PHPUnit\Framework\assertIsIterable')) {
    /**
     * Asserts that a variable is of type iterable.
     *
     * @throws Exception
     * @throws ExpectationFailedException
     *
     * @phpstan-assert iterable<mixed> $actual
     *
     * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
     *
     * @see Assert::assertIsIterable
     */
    function assertIsIterable(mixed $actual, string $message = ''): void
    {}
}

if (!function_exists('PHPUnit\Framework\assertIsNotArray')) {
    /**
     * Asserts that a variable is not of type array.
     *
     * @throws Exception
     * @throws ExpectationFailedException
     *
     * @phpstan-assert !array<mixed> $actual
     *
     * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
     *
     * @see Assert::assertIsNotArray
     */
    function assertIsNotArray(mixed $actual, string $message = ''): void
    {}
}

if (!function_exists('PHPUnit\Framework\assertIsNotBool')) {
    /**
     * Asserts that a variable is not of type bool.
     *
     * @throws Exception
     * @throws ExpectationFailedException
     *
     * @phpstan-assert !bool $actual
     *
     * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
     *
     * @see Assert::assertIsNotBool
     */
    function assertIsNotBool(mixed $actual, string $message = ''): void
    {}
}

if (!function_exists('PHPUnit\Framework\assertIsNotFloat')) {
    /**
     * Asserts that a variable is not of type float.
     *
     * @throws Exception
     * @throws ExpectationFailedException
     *
     * @phpstan-assert !float $actual
     *
     * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
     *
     * @see Assert::assertIsNotFloat
     */
    function assertIsNotFloat(mixed $actual, string $message = ''): void
    {}
}

if (!function_exists('PHPUnit\Framework\assertIsNotInt')) {
    /**
     * Asserts that a variable is not of type int.
     *
     * @throws Exception
     * @throws ExpectationFailedException
     *
     * @phpstan-assert !int $actual
     *
     * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
     *
     * @see Assert::assertIsNotInt
     */
    function assertIsNotInt(mixed $actual, string $message = ''): void
    {}
}

if (!function_exists('PHPUnit\Framework\assertIsNotNumeric')) {
    /**
     * Asserts that a variable is not of type numeric.
     *
     * @throws Exception
     * @throws ExpectationFailedException
     *
     * @phpstan-assert !numeric $actual
     *
     * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
     *
     * @see Assert::assertIsNotNumeric
     */
    function assertIsNotNumeric(mixed $actual, string $message = ''): void
    {}
}

if (!function_exists('PHPUnit\Framework\assertIsNotObject')) {
    /**
     * Asserts that a variable is not of type object.
     *
     * @throws Exception
     * @throws ExpectationFailedException
     *
     * @phpstan-assert !object $actual
     *
     * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
     *
     * @see Assert::assertIsNotObject
     */
    function assertIsNotObject(mixed $actual, string $message = ''): void
    {}
}

if (!function_exists('PHPUnit\Framework\assertIsNotResource')) {
    /**
     * Asserts that a variable is not of type resource.
     *
     * @throws Exception
     * @throws ExpectationFailedException
     *
     * @phpstan-assert !resource $actual
     *
     * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
     *
     * @see Assert::assertIsNotResource
     */
    function assertIsNotResource(mixed $actual, string $message = ''): void
    {}
}

if (!function_exists('PHPUnit\Framework\assertIsNotClosedResource')) {
    /**
     * Asserts that a variable is not of type resource.
     *
     * @throws Exception
     * @throws ExpectationFailedException
     *
     * @phpstan-assert !resource $actual
     *
     * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
     *
     * @see Assert::assertIsNotClosedResource
     */
    function assertIsNotClosedResource(mixed $actual, string $message = ''): void
    {}
}

if (!function_exists('PHPUnit\Framework\assertIsNotString')) {
    /**
     * Asserts that a variable is not of type string.
     *
     * @throws Exception
     * @throws ExpectationFailedException
     *
     * @phpstan-assert !string $actual
     *
     * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
     *
     * @see Assert::assertIsNotString
     */
    function assertIsNotString(mixed $actual, string $message = ''): void
    {}
}

if (!function_exists('PHPUnit\Framework\assertIsNotScalar')) {
    /**
     * Asserts that a variable is not of type scalar.
     *
     * @throws Exception
     * @throws ExpectationFailedException
     *
     * @phpstan-assert !scalar $actual
     *
     * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
     *
     * @see Assert::assertIsNotScalar
     */
    function assertIsNotScalar(mixed $actual, string $message = ''): void
    {}
}

if (!function_exists('PHPUnit\Framework\assertIsNotCallable')) {
    /**
     * Asserts that a variable is not of type callable.
     *
     * @throws Exception
     * @throws ExpectationFailedException
     *
     * @phpstan-assert !callable $actual
     *
     * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
     *
     * @see Assert::assertIsNotCallable
     */
    function assertIsNotCallable(mixed $actual, string $message = ''): void
    {}
}

if (!function_exists('PHPUnit\Framework\assertIsNotIterable')) {
    /**
     * Asserts that a variable is not of type iterable.
     *
     * @throws Exception
     * @throws ExpectationFailedException
     *
     * @phpstan-assert !iterable<mixed> $actual
     *
     * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
     *
     * @see Assert::assertIsNotIterable
     */
    function assertIsNotIterable(mixed $actual, string $message = ''): void
    {}
}

if (!function_exists('PHPUnit\Framework\assertMatchesRegularExpression')) {
    /**
     * Asserts that a string matches a given regular expression.
     *
     * @throws ExpectationFailedException
     *
     * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
     *
     * @see Assert::assertMatchesRegularExpression
     */
    function assertMatchesRegularExpression(string $pattern, string $string, string $message = ''): void
    {}
}

if (!function_exists('PHPUnit\Framework\assertDoesNotMatchRegularExpression')) {
    /**
     * Asserts that a string does not match a given regular expression.
     *
     * @throws ExpectationFailedException
     *
     * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
     *
     * @see Assert::assertDoesNotMatchRegularExpression
     */
    function assertDoesNotMatchRegularExpression(string $pattern, string $string, string $message = ''): void
    {}
}

if (!function_exists('PHPUnit\Framework\assertSameSize')) {
    /**
     * Assert that the size of two arrays (or `Countable` or `Traversable` objects)
     * is the same.
     *
     * @param Countable|iterable<mixed> $expected
     * @param Countable|iterable<mixed> $actual
     *
     * @throws Exception
     * @throws ExpectationFailedException
     * @throws GeneratorNotSupportedException
     *
     * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
     *
     * @see Assert::assertSameSize
     */
    function assertSameSize(Countable|iterable $expected, Countable|iterable $actual, string $message = ''): void
    {}
}

if (!function_exists('PHPUnit\Framework\assertNotSameSize')) {
    /**
     * Assert that the size of two arrays (or `Countable` or `Traversable` objects)
     * is not the same.
     *
     * @param Countable|iterable<mixed> $expected
     * @param Countable|iterable<mixed> $actual
     *
     * @throws Exception
     * @throws ExpectationFailedException
     * @throws GeneratorNotSupportedException
     *
     * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
     *
     * @see Assert::assertNotSameSize
     */
    function assertNotSameSize(Countable|iterable $expected, Countable|iterable $actual, string $message = ''): void
    {}
}

if (!function_exists('PHPUnit\Framework\assertStringContainsStringIgnoringLineEndings')) {
    /**
     * @throws ExpectationFailedException
     *
     * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
     *
     * @see Assert::assertStringContainsStringIgnoringLineEndings
     */
    function assertStringContainsStringIgnoringLineEndings(string $needle, string $haystack, string $message = ''): void
    {}
}

if (!function_exists('PHPUnit\Framework\assertStringEqualsStringIgnoringLineEndings')) {
    /**
     * Asserts that two strings are equal except for line endings.
     *
     * @throws ExpectationFailedException
     *
     * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
     *
     * @see Assert::assertStringEqualsStringIgnoringLineEndings
     */
    function assertStringEqualsStringIgnoringLineEndings(string $expected, string $actual, string $message = ''): void
    {}
}

if (!function_exists('PHPUnit\Framework\assertStringEqualsStringIgnoringWhitespace')) {
    /**
     * Asserts that two strings are equal ignoring whitespace.
     *
     * @throws ExpectationFailedException
     *
     * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
     *
     * @see Assert::assertStringEqualsStringIgnoringWhitespace
     */
    function assertStringEqualsStringIgnoringWhitespace(string $expected, string $actual, string $message = ''): void
    {}
}

if (!function_exists('PHPUnit\Framework\assertStringNotEqualsStringIgnoringWhitespace')) {
    /**
     * Asserts that two strings are not equal ignoring whitespace.
     *
     * @throws ExpectationFailedException
     *
     * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
     *
     * @see Assert::assertStringNotEqualsStringIgnoringWhitespace
     */
    function assertStringNotEqualsStringIgnoringWhitespace(string $expected, string $actual, string $message = ''): void
    {}
}

if (!function_exists('PHPUnit\Framework\assertFileMatchesFormat')) {
    /**
     * Asserts that a string matches a given format string.
     *
     * @throws ExpectationFailedException
     *
     * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
     *
     * @see Assert::assertFileMatchesFormat
     */
    function assertFileMatchesFormat(string $format, string $actualFile, string $message = ''): void
    {}
}

if (!function_exists('PHPUnit\Framework\assertFileMatchesFormatFile')) {
    /**
     * Asserts that a string matches a given format string.
     *
     * @throws ExpectationFailedException
     *
     * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
     *
     * @see Assert::assertFileMatchesFormatFile
     */
    function assertFileMatchesFormatFile(string $formatFile, string $actualFile, string $message = ''): void
    {}
}

if (!function_exists('PHPUnit\Framework\assertStringMatchesFormat')) {
    /**
     * Asserts that a string matches a given format string.
     *
     * @throws ExpectationFailedException
     *
     * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
     *
     * @see Assert::assertStringMatchesFormat
     */
    function assertStringMatchesFormat(string $format, string $string, string $message = ''): void
    {}
}

if (!function_exists('PHPUnit\Framework\assertStringMatchesFormatFile')) {
    /**
     * Asserts that a string matches a given format file.
     *
     * @throws ExpectationFailedException
     *
     * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
     *
     * @see Assert::assertStringMatchesFormatFile
     */
    function assertStringMatchesFormatFile(string $formatFile, string $string, string $message = ''): void
    {}
}

if (!function_exists('PHPUnit\Framework\assertStringStartsWith')) {
    /**
     * Asserts that a string starts with a given prefix.
     *
     * @param non-empty-string $prefix
     *
     * @throws ExpectationFailedException
     * @throws InvalidArgumentException
     *
     * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
     *
     * @see Assert::assertStringStartsWith
     */
    function assertStringStartsWith(string $prefix, string $string, string $message = ''): void
    {}
}

if (!function_exists('PHPUnit\Framework\assertStringStartsNotWith')) {
    /**
     * Asserts that a string starts not with a given prefix.
     *
     * @param non-empty-string $prefix
     *
     * @throws ExpectationFailedException
     * @throws InvalidArgumentException
     *
     * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
     *
     * @see Assert::assertStringStartsNotWith
     */
    function assertStringStartsNotWith(string $prefix, string $string, string $message = ''): void
    {}
}

if (!function_exists('PHPUnit\Framework\assertStringContainsString')) {
    /**
     * @throws ExpectationFailedException
     *
     * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
     *
     * @see Assert::assertStringContainsString
     */
    function assertStringContainsString(string $needle, string $haystack, string $message = ''): void
    {}
}

if (!function_exists('PHPUnit\Framework\assertStringContainsStringIgnoringCase')) {
    /**
     * @throws ExpectationFailedException
     *
     * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
     *
     * @see Assert::assertStringContainsStringIgnoringCase
     */
    function assertStringContainsStringIgnoringCase(string $needle, string $haystack, string $message = ''): void
    {}
}

if (!function_exists('PHPUnit\Framework\assertStringNotContainsString')) {
    /**
     * @throws ExpectationFailedException
     *
     * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
     *
     * @see Assert::assertStringNotContainsString
     */
    function assertStringNotContainsString(string $needle, string $haystack, string $message = ''): void
    {}
}

if (!function_exists('PHPUnit\Framework\assertStringNotContainsStringIgnoringCase')) {
    /**
     * @throws ExpectationFailedException
     *
     * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
     *
     * @see Assert::assertStringNotContainsStringIgnoringCase
     */
    function assertStringNotContainsStringIgnoringCase(string $needle, string $haystack, string $message = ''): void
    {}
}

if (!function_exists('PHPUnit\Framework\assertStringEndsWith')) {
    /**
     * Asserts that a string ends with a given suffix.
     *
     * @param non-empty-string $suffix
     *
     * @throws ExpectationFailedException
     * @throws InvalidArgumentException
     *
     * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
     *
     * @see Assert::assertStringEndsWith
     */
    function assertStringEndsWith(string $suffix, string $string, string $message = ''): void
    {}
}

if (!function_exists('PHPUnit\Framework\assertStringEndsNotWith')) {
    /**
     * Asserts that a string ends not with a given suffix.
     *
     * @param non-empty-string $suffix
     *
     * @throws ExpectationFailedException
     * @throws InvalidArgumentException
     *
     * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
     *
     * @see Assert::assertStringEndsNotWith
     */
    function assertStringEndsNotWith(string $suffix, string $string, string $message = ''): void
    {}
}

if (!function_exists('PHPUnit\Framework\assertXmlFileEqualsXmlFile')) {
    /**
     * Asserts that two XML files are equal, ignoring comments.
     *
     * @throws Exception
     * @throws ExpectationFailedException
     * @throws XmlException
     *
     * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
     *
     * @see Assert::assertXmlFileEqualsXmlFile
     */
    function assertXmlFileEqualsXmlFile(string $expectedFile, string $actualFile, string $message = ''): void
    {}
}

if (!function_exists('PHPUnit\Framework\assertXmlFileNotEqualsXmlFile')) {
    /**
     * Asserts that two XML files are not equal, ignoring comments.
     *
     * @throws \PHPUnit\Util\Exception
     * @throws ExpectationFailedException
     *
     * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
     *
     * @see Assert::assertXmlFileNotEqualsXmlFile
     */
    function assertXmlFileNotEqualsXmlFile(string $expectedFile, string $actualFile, string $message = ''): void
    {}
}

if (!function_exists('PHPUnit\Framework\assertXmlStringEqualsXmlFile')) {
    /**
     * Asserts that two XML documents are equal, ignoring comments.
     *
     * @throws ExpectationFailedException
     * @throws XmlException
     *
     * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
     *
     * @see Assert::assertXmlStringEqualsXmlFile
     */
    function assertXmlStringEqualsXmlFile(string $expectedFile, string $actualXml, string $message = ''): void
    {}
}

if (!function_exists('PHPUnit\Framework\assertXmlStringNotEqualsXmlFile')) {
    /**
     * Asserts that two XML documents are not equal, ignoring comments.
     *
     * @throws ExpectationFailedException
     * @throws XmlException
     *
     * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
     *
     * @see Assert::assertXmlStringNotEqualsXmlFile
     */
    function assertXmlStringNotEqualsXmlFile(string $expectedFile, string $actualXml, string $message = ''): void
    {}
}

if (!function_exists('PHPUnit\Framework\assertXmlStringEqualsXmlString')) {
    /**
     * Asserts that two XML documents are equal, ignoring comments.
     *
     * @throws ExpectationFailedException
     * @throws XmlException
     *
     * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
     *
     * @see Assert::assertXmlStringEqualsXmlString
     */
    function assertXmlStringEqualsXmlString(string $expectedXml, string $actualXml, string $message = ''): void
    {}
}

if (!function_exists('PHPUnit\Framework\assertXmlStringNotEqualsXmlString')) {
    /**
     * Asserts that two XML documents are not equal, ignoring comments.
     *
     * @throws ExpectationFailedException
     * @throws XmlException
     *
     * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
     *
     * @see Assert::assertXmlStringNotEqualsXmlString
     */
    function assertXmlStringNotEqualsXmlString(string $expectedXml, string $actualXml, string $message = ''): void
    {}
}

if (!function_exists('PHPUnit\Framework\assertXmlFileEqualsXmlFileConsideringComments')) {
    /**
     * Asserts that two XML files are equal, considering comments.
     *
     * @throws Exception
     * @throws ExpectationFailedException
     * @throws XmlException
     *
     * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
     *
     * @see Assert::assertXmlFileEqualsXmlFileConsideringComments
     */
    function assertXmlFileEqualsXmlFileConsideringComments(string $expectedFile, string $actualFile, string $message = ''): void
    {}
}

if (!function_exists('PHPUnit\Framework\assertXmlFileNotEqualsXmlFileConsideringComments')) {
    /**
     * Asserts that two XML files are not equal, considering comments.
     *
     * @throws \PHPUnit\Util\Exception
     * @throws ExpectationFailedException
     *
     * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
     *
     * @see Assert::assertXmlFileNotEqualsXmlFileConsideringComments
     */
    function assertXmlFileNotEqualsXmlFileConsideringComments(string $expectedFile, string $actualFile, string $message = ''): void
    {}
}

if (!function_exists('PHPUnit\Framework\assertXmlStringEqualsXmlFileConsideringComments')) {
    /**
     * Asserts that two XML documents are equal, considering comments.
     *
     * @throws ExpectationFailedException
     * @throws XmlException
     *
     * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
     *
     * @see Assert::assertXmlStringEqualsXmlFileConsideringComments
     */
    function assertXmlStringEqualsXmlFileConsideringComments(string $expectedFile, string $actualXml, string $message = ''): void
    {}
}

if (!function_exists('PHPUnit\Framework\assertXmlStringNotEqualsXmlFileConsideringComments')) {
    /**
     * Asserts that two XML documents are not equal, considering comments.
     *
     * @throws ExpectationFailedException
     * @throws XmlException
     *
     * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
     *
     * @see Assert::assertXmlStringNotEqualsXmlFileConsideringComments
     */
    function assertXmlStringNotEqualsXmlFileConsideringComments(string $expectedFile, string $actualXml, string $message = ''): void
    {}
}

if (!function_exists('PHPUnit\Framework\assertXmlStringEqualsXmlStringConsideringComments')) {
    /**
     * Asserts that two XML documents are equal, considering comments.
     *
     * @throws ExpectationFailedException
     * @throws XmlException
     *
     * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
     *
     * @see Assert::assertXmlStringEqualsXmlStringConsideringComments
     */
    function assertXmlStringEqualsXmlStringConsideringComments(string $expectedXml, string $actualXml, string $message = ''): void
    {}
}

if (!function_exists('PHPUnit\Framework\assertXmlStringNotEqualsXmlStringConsideringComments')) {
    /**
     * Asserts that two XML documents are not equal, considering comments.
     *
     * @throws ExpectationFailedException
     * @throws XmlException
     *
     * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
     *
     * @see Assert::assertXmlStringNotEqualsXmlStringConsideringComments
     */
    function assertXmlStringNotEqualsXmlStringConsideringComments(string $expectedXml, string $actualXml, string $message = ''): void
    {}
}

if (!function_exists('PHPUnit\Framework\assertThat')) {
    /**
     * Evaluates a PHPUnit\Framework\Constraint matcher object.
     *
     * @throws ExpectationFailedException
     *
     * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
     *
     * @see Assert::assertThat
     */
    function assertThat(mixed $value, Constraint $constraint, string $message = ''): void
    {}
}

if (!function_exists('PHPUnit\Framework\assertJson')) {
    /**
     * Asserts that a string is a valid JSON string.
     *
     * @throws ExpectationFailedException
     *
     * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
     *
     * @see Assert::assertJson
     */
    function assertJson(string $actual, string $message = ''): void
    {}
}

if (!function_exists('PHPUnit\Framework\assertJsonStringEqualsJsonString')) {
    /**
     * Asserts that two given JSON encoded objects or arrays are equal.
     *
     * @throws ExpectationFailedException
     *
     * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
     *
     * @see Assert::assertJsonStringEqualsJsonString
     */
    function assertJsonStringEqualsJsonString(string $expectedJson, string $actualJson, string $message = ''): void
    {}
}

if (!function_exists('PHPUnit\Framework\assertJsonStringNotEqualsJsonString')) {
    /**
     * Asserts that two given JSON encoded objects or arrays are not equal.
     *
     * @throws ExpectationFailedException
     *
     * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
     *
     * @see Assert::assertJsonStringNotEqualsJsonString
     */
    function assertJsonStringNotEqualsJsonString(string $expectedJson, string $actualJson, string $message = ''): void
    {}
}

if (!function_exists('PHPUnit\Framework\assertJsonStringEqualsJsonFile')) {
    /**
     * Asserts that the generated JSON encoded object and the content of the given file are equal.
     *
     * @throws ExpectationFailedException
     *
     * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
     *
     * @see Assert::assertJsonStringEqualsJsonFile
     */
    function assertJsonStringEqualsJsonFile(string $expectedFile, string $actualJson, string $message = ''): void
    {}
}

if (!function_exists('PHPUnit\Framework\assertJsonStringNotEqualsJsonFile')) {
    /**
     * Asserts that the generated JSON encoded object and the content of the given file are not equal.
     *
     * @throws ExpectationFailedException
     *
     * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
     *
     * @see Assert::assertJsonStringNotEqualsJsonFile
     */
    function assertJsonStringNotEqualsJsonFile(string $expectedFile, string $actualJson, string $message = ''): void
    {}
}

if (!function_exists('PHPUnit\Framework\assertJsonFileEqualsJsonFile')) {
    /**
     * Asserts that two JSON files are equal.
     *
     * @throws ExpectationFailedException
     *
     * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
     *
     * @see Assert::assertJsonFileEqualsJsonFile
     */
    function assertJsonFileEqualsJsonFile(string $expectedFile, string $actualFile, string $message = ''): void
    {}
}

if (!function_exists('PHPUnit\Framework\assertJsonFileNotEqualsJsonFile')) {
    /**
     * Asserts that two JSON files are not equal.
     *
     * @throws ExpectationFailedException
     *
     * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
     *
     * @see Assert::assertJsonFileNotEqualsJsonFile
     */
    function assertJsonFileNotEqualsJsonFile(string $expectedFile, string $actualFile, string $message = ''): void
    {}
}

if (!function_exists('PHPUnit\Framework\logicalAnd')) {
    /**
     * @throws Exception
     */
    function logicalAnd(mixed ...$constraints): LogicalAnd
    {}
}

if (!function_exists('PHPUnit\Framework\logicalOr')) {
    function logicalOr(mixed ...$constraints): LogicalOr
    {}
}

if (!function_exists('PHPUnit\Framework\logicalNot')) {
    function logicalNot(Constraint $constraint): LogicalNot
    {}
}

if (!function_exists('PHPUnit\Framework\logicalXor')) {
    function logicalXor(mixed ...$constraints): LogicalXor
    {}
}

if (!function_exists('PHPUnit\Framework\anything')) {
    function anything(): IsAnything
    {}
}

if (!function_exists('PHPUnit\Framework\isTrue')) {
    function isTrue(): IsTrue
    {}
}

if (!function_exists('PHPUnit\Framework\isFalse')) {
    function isFalse(): IsFalse
    {}
}

if (!function_exists('PHPUnit\Framework\isJson')) {
    function isJson(): IsJson
    {}
}

if (!function_exists('PHPUnit\Framework\isNull')) {
    function isNull(): IsNull
    {}
}

if (!function_exists('PHPUnit\Framework\isFinite')) {
    function isFinite(): IsFinite
    {}
}

if (!function_exists('PHPUnit\Framework\isInfinite')) {
    function isInfinite(): IsInfinite
    {}
}

if (!function_exists('PHPUnit\Framework\isNan')) {
    function isNan(): IsNan
    {}
}

if (!function_exists('PHPUnit\Framework\containsEqual')) {
    function containsEqual(mixed $value): TraversableContainsEqual
    {}
}

if (!function_exists('PHPUnit\Framework\containsIdentical')) {
    function containsIdentical(mixed $value): TraversableContainsIdentical
    {}
}

if (!function_exists('PHPUnit\Framework\containsOnlyArray')) {
    function containsOnlyArray(): TraversableContainsOnly
    {}
}

if (!function_exists('PHPUnit\Framework\containsOnlyBool')) {
    function containsOnlyBool(): TraversableContainsOnly
    {}
}

if (!function_exists('PHPUnit\Framework\containsOnlyCallable')) {
    function containsOnlyCallable(): TraversableContainsOnly
    {}
}

if (!function_exists('PHPUnit\Framework\containsOnlyFloat')) {
    function containsOnlyFloat(): TraversableContainsOnly
    {}
}

if (!function_exists('PHPUnit\Framework\containsOnlyInt')) {
    function containsOnlyInt(): TraversableContainsOnly
    {}
}

if (!function_exists('PHPUnit\Framework\containsOnlyIterable')) {
    function containsOnlyIterable(): TraversableContainsOnly
    {}
}

if (!function_exists('PHPUnit\Framework\containsOnlyNull')) {
    function containsOnlyNull(): TraversableContainsOnly
    {}
}

if (!function_exists('PHPUnit\Framework\containsOnlyNumeric')) {
    function containsOnlyNumeric(): TraversableContainsOnly
    {}
}

if (!function_exists('PHPUnit\Framework\containsOnlyObject')) {
    function containsOnlyObject(): TraversableContainsOnly
    {}
}

if (!function_exists('PHPUnit\Framework\containsOnlyResource')) {
    function containsOnlyResource(): TraversableContainsOnly
    {}
}

if (!function_exists('PHPUnit\Framework\containsOnlyClosedResource')) {
    function containsOnlyClosedResource(): TraversableContainsOnly
    {}
}

if (!function_exists('PHPUnit\Framework\containsOnlyScalar')) {
    function containsOnlyScalar(): TraversableContainsOnly
    {}
}

if (!function_exists('PHPUnit\Framework\containsOnlyString')) {
    function containsOnlyString(): TraversableContainsOnly
    {}
}

if (!function_exists('PHPUnit\Framework\containsOnlyInstancesOf')) {
    /**
     * @param class-string $className
     *
     * @throws Exception
     */
    function containsOnlyInstancesOf(string $className): TraversableContainsOnly
    {}
}

if (!function_exists('PHPUnit\Framework\arrayHasKey')) {
    function arrayHasKey(mixed $key): ArrayHasKey
    {}
}

if (!function_exists('PHPUnit\Framework\isList')) {
    function isList(): IsList
    {}
}

if (!function_exists('PHPUnit\Framework\equalTo')) {
    function equalTo(mixed $value): IsEqual
    {}
}

if (!function_exists('PHPUnit\Framework\equalToCanonicalizing')) {
    function equalToCanonicalizing(mixed $value): IsEqualCanonicalizing
    {}
}

if (!function_exists('PHPUnit\Framework\equalToIgnoringCase')) {
    function equalToIgnoringCase(mixed $value): IsEqualIgnoringCase
    {}
}

if (!function_exists('PHPUnit\Framework\equalToWithDelta')) {
    function equalToWithDelta(mixed $value, float $delta): IsEqualWithDelta
    {}
}

if (!function_exists('PHPUnit\Framework\isEmpty')) {
    function isEmpty(): IsEmpty
    {}
}

if (!function_exists('PHPUnit\Framework\isWritable')) {
    function isWritable(): IsWritable
    {}
}

if (!function_exists('PHPUnit\Framework\isReadable')) {
    function isReadable(): IsReadable
    {}
}

if (!function_exists('PHPUnit\Framework\directoryExists')) {
    function directoryExists(): DirectoryExists
    {}
}

if (!function_exists('PHPUnit\Framework\fileExists')) {
    function fileExists(): FileExists
    {}
}

if (!function_exists('PHPUnit\Framework\greaterThan')) {
    function greaterThan(mixed $value): GreaterThan
    {}
}

if (!function_exists('PHPUnit\Framework\greaterThanOrEqual')) {
    function greaterThanOrEqual(mixed $value): LogicalOr
    {}
}

if (!function_exists('PHPUnit\Framework\identicalTo')) {
    function identicalTo(mixed $value): IsIdentical
    {}
}

if (!function_exists('PHPUnit\Framework\isInstanceOf')) {
    /**
     * @throws UnknownClassOrInterfaceException
     */
    function isInstanceOf(string $className): IsInstanceOf
    {}
}

if (!function_exists('PHPUnit\Framework\isArray')) {
    function isArray(): IsType
    {}
}

if (!function_exists('PHPUnit\Framework\isBool')) {
    function isBool(): IsType
    {}
}

if (!function_exists('PHPUnit\Framework\isCallable')) {
    function isCallable(): IsType
    {}
}

if (!function_exists('PHPUnit\Framework\isFloat')) {
    function isFloat(): IsType
    {}
}

if (!function_exists('PHPUnit\Framework\isInt')) {
    function isInt(): IsType
    {}
}

if (!function_exists('PHPUnit\Framework\isIterable')) {
    function isIterable(): IsType
    {}
}

if (!function_exists('PHPUnit\Framework\isNumeric')) {
    function isNumeric(): IsType
    {}
}

if (!function_exists('PHPUnit\Framework\isObject')) {
    function isObject(): IsType
    {}
}

if (!function_exists('PHPUnit\Framework\isResource')) {
    function isResource(): IsType
    {}
}

if (!function_exists('PHPUnit\Framework\isClosedResource')) {
    function isClosedResource(): IsType
    {}
}

if (!function_exists('PHPUnit\Framework\isScalar')) {
    function isScalar(): IsType
    {}
}

if (!function_exists('PHPUnit\Framework\isString')) {
    function isString(): IsType
    {}
}

if (!function_exists('PHPUnit\Framework\lessThan')) {
    function lessThan(mixed $value): LessThan
    {}
}

if (!function_exists('PHPUnit\Framework\lessThanOrEqual')) {
    function lessThanOrEqual(mixed $value): LogicalOr
    {}
}

if (!function_exists('PHPUnit\Framework\matchesRegularExpression')) {
    function matchesRegularExpression(string $pattern): RegularExpression
    {}
}

if (!function_exists('PHPUnit\Framework\matches')) {
    function matches(string $string): StringMatchesFormatDescription
    {}
}

if (!function_exists('PHPUnit\Framework\stringStartsWith')) {
    /**
     * @param non-empty-string $prefix
     *
     * @throws InvalidArgumentException
     */
    function stringStartsWith(string $prefix): StringStartsWith
    {}
}

if (!function_exists('PHPUnit\Framework\stringContains')) {
    function stringContains(string $string, bool $case = true): StringContains
    {}
}

if (!function_exists('PHPUnit\Framework\stringEndsWith')) {
    /**
     * @param non-empty-string $suffix
     *
     * @throws InvalidArgumentException
     */
    function stringEndsWith(string $suffix): StringEndsWith
    {}
}

if (!function_exists('PHPUnit\Framework\stringEqualsStringIgnoringLineEndings')) {
    function stringEqualsStringIgnoringLineEndings(string $string): StringEqualsStringIgnoringLineEndings
    {}
}

if (!function_exists('PHPUnit\Framework\stringEqualsStringIgnoringWhitespace')) {
    function stringEqualsStringIgnoringWhitespace(string $string): StringEqualsStringIgnoringWhitespace
    {}
}

if (!function_exists('PHPUnit\Framework\countOf')) {
    function countOf(int $count): Count
    {}
}

if (!function_exists('PHPUnit\Framework\objectEquals')) {
    function objectEquals(object $object, string $method = 'equals'): ObjectEquals
    {}
}

if (!function_exists('PHPUnit\Framework\callback')) {
    /**
     * @template CallbackInput of mixed
     *
     * @param callable(CallbackInput $callback): bool $callback
     *
     * @return Callback<CallbackInput>
     */
    function callback(callable $callback): Callback
    {}
}

if (!function_exists('PHPUnit\Framework\any')) {
    /**
     * Returns a matcher that matches when the method is executed
     * zero or more times.
     *
     * @deprecated https://github.com/sebastianbergmann/phpunit/issues/6461
     */
    function any(): AnyInvokedCountMatcher
    {}
}

if (!function_exists('PHPUnit\Framework\never')) {
    /**
     * Returns a matcher that matches when the method is never executed.
     */
    function never(): InvokedCountMatcher
    {}
}

if (!function_exists('PHPUnit\Framework\atLeast')) {
    /**
     * Returns a matcher that matches when the method is executed
     * at least N times.
     */
    function atLeast(int $requiredInvocations): InvokedAtLeastCountMatcher
    {}
}

if (!function_exists('PHPUnit\Framework\atLeastOnce')) {
    /**
     * Returns a matcher that matches when the method is executed at least once.
     */
    function atLeastOnce(): InvokedAtLeastOnceMatcher
    {}
}

if (!function_exists('PHPUnit\Framework\once')) {
    /**
     * Returns a matcher that matches when the method is executed exactly once.
     */
    function once(): InvokedCountMatcher
    {}
}

if (!function_exists('PHPUnit\Framework\exactly')) {
    /**
     * Returns a matcher that matches when the method is executed
     * exactly $count times.
     */
    function exactly(int $count): InvokedCountMatcher
    {}
}

if (!function_exists('PHPUnit\Framework\atMost')) {
    /**
     * Returns a matcher that matches when the method is executed
     * at most N times.
     */
    function atMost(int $allowedInvocations): InvokedAtMostCountMatcher
    {}
}

if (!function_exists('PHPUnit\Framework\throwException')) {
    function throwException(Throwable $exception): ExceptionStub
    {}
}
