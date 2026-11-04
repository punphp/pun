<?php

/**
 * Mockery (https://docs.mockery.io/en/stable/)
 *
 * @copyright https://github.com/mockery/mockery/blob/HEAD/COPYRIGHT.md
 * @license   https://github.com/mockery/mockery/blob/HEAD/LICENSE BSD 3-Clause License
 * @see       https://github.com/mockery/mockery for the canonical source repository
 */

namespace Mockery;

use Closure;
use Hamcrest\Matcher;
use Hamcrest_Matcher;
use InvalidArgumentException;
use Mockery;
use Mockery\CountValidator\AtLeast;
use Mockery\CountValidator\AtMost;
use Mockery\CountValidator\Exact;
use Mockery\Matcher\AndAnyOtherArgs;
use Mockery\Matcher\AnyArgs;
use Mockery\Matcher\ArgumentListMatcher;
use Mockery\Matcher\MatcherInterface;
use Mockery\Matcher\MultiArgumentClosure;
use Mockery\Matcher\NoArgs;
use OutOfBoundsException;
use Override;
use PHPUnit\Framework\Constraint\Constraint;
use ReflectionException;
use ReflectionMethod;
use ReturnTypeWillChange;
use Throwable;

use const E_USER_DEPRECATED;

use function array_is_list;
use function array_key_exists;
use function array_merge;
use function array_search;
use function array_shift;
use function array_slice;
use function count;
use function current;
use function func_get_args;
use function get_class;
use function in_array;
use function is_array;
use function is_int;
use function is_object;
use function is_string;
use function sprintf;

use function trigger_error;

class Expectation implements ExpectationInterface
{
    public const ERROR_ZERO_INVOCATION = 'shouldNotReceive(), never(), times(0) chaining additional invocation count methods has been deprecated and will throw an exception in a future version of Mockery';

    /**
     * Actual count of calls to this expectation
     *
     * @var int
     */
    protected $_actualCount = 0;

    /**
     * Exception message
     *
     * @var null|string
     */
    protected $_because;

    /**
     * Array of closures executed with given arguments to generate a result
     * to be returned
     *
     * @var array
     */
    protected $_closureQueue = [];

    /**
     * The count validator class to use
     *
     * @var string
     */
    protected $_countValidatorClass = Exact::class;

    /**
     * Count validator store
     *
     * @var array
     */
    protected $_countValidators = [];

    /**
     * Arguments expected by this expectation
     *
     * @var array
     */
    protected $_expectedArgs = [];

    /**
     * Expected count of calls to this expectation
     *
     * @var int
     */
    protected $_expectedCount = -1;

    /**
     * Integer representing the call order of this expectation on a global basis
     *
     * @var null|int
     */
    protected $_globalOrderNumber;

    /**
     * Flag indicating whether the order of calling is determined locally or
     * globally
     *
     * @var bool
     */
    protected $_globally = false;

    /**
     * Mock object to which this expectation belongs
     *
     * @var MockInterface
     */
    protected $_mock;

    /**
     * Method name
     *
     * @var string
     */
    protected $_name;

    /**
     * Integer representing the call order of this expectation
     *
     * @var null|int
     */
    protected $_orderNumber;

    /**
     * Flag indicating if the return value should be obtained from the original
     * class method instead of returning predefined values from the return queue
     *
     * @var bool
     */
    protected $_passthru = false;

    /**
     * Array of return values as a queue for multiple return sequence
     *
     * @var array
     */
    protected $_returnQueue = [];

    /**
     * Value to return from this expectation
     *
     * @var mixed
     */
    protected $_returnValue;

    /**
     * Array of values to be set when this expectation matches
     *
     * @var array
     */
    protected $_setQueue = [];

    /**
     * Flag indicating that an exception is expected to be throw (not returned)
     *
     * @var bool
     */
    protected $_throw = false;

    /**
     * @param MockInterface $mock
     */
    public function __construct(LegacyMockInterface $mock, string $name)
    {}

    /**
     * Cloning logic
     */
    public function __clone()
    {}

    /**
     * Return a string with the method name and arguments formatted
     *
     * @return string
     */
    #[ReturnTypeWillChange]
    public function __toString()
    {}

    /**
     * Set a return value, or sequential queue of return values
     *
     * @param  mixed ...$args
     * @return self
     */
    #[Override]
    public function andReturn(...$args)
    {}

    /**
     * Sets up a closure to return the nth argument from the expected method call
     *
     * @param  int  $index
     * @return self
     */
    public function andReturnArg($index)
    {}

    /**
     * @return self
     */
    public function andReturnFalse()
    {}

    /**
     * Return null. This is merely a language construct for Mock describing.
     *
     * @return self
     */
    public function andReturnNull()
    {}

    /**
     * Return this mock, like a fluent interface
     *
     * @return self
     */
    public function andReturnSelf()
    {}

    /**
     * @return self
     */
    public function andReturnTrue()
    {}

    /**
     * Return a self-returning black hole object.
     *
     * @return self
     */
    public function andReturnUndefined()
    {}

    /**
     * Set a closure or sequence of closures with which to generate return
     * values. The arguments passed to the expected method are passed to the
     * closures as parameters.
     *
     * @param  callable ...$args
     * @return self
     */
    public function andReturnUsing(...$args)
    {}

    /**
     * Set a sequential queue of return values with an array
     *
     * @param  array<mixed> $values
     * @return self
     */
    public function andReturnValues(array $values)
    {}

    /**
     * Set a return value, or sequential queue of return values
     *
     * @param  mixed ...$args
     * @return self
     */
    #[Override]
    public function andReturns(...$args)
    {}

    /**
     * Register values to be set to a public property each time this expectation occurs
     *
     * @param  string       $name
     * @param  array<mixed> ...$values
     * @return self
     */
    public function andSet($name, ...$values)
    {}

    /**
     * Set Exception class and arguments to that class to be thrown
     *
     * @param  string|Throwable $exception
     * @param  string           $message
     * @param  int              $code
     * @return self
     */
    public function andThrow($exception, $message = '', $code = 0, ?\Exception $previous = null)
    {}

    /**
     * Set Exception classes to be thrown
     *
     * @param  array<Throwable> $exceptions
     * @return self
     */
    public function andThrowExceptions(array $exceptions)
    {}

    /**
     * Set Exception class and arguments to that class to be thrown
     *
     * @param  string|Throwable $exception
     * @param  string           $message
     * @param  int              $code
     * @return self
     */
    public function andThrows($exception, $message = '', $code = 0, ?\Exception $previous = null)
    {}

    /**
     * Sets up a closure that will yield each of the provided args
     *
     * @param  mixed ...$args
     * @return self
     */
    public function andYield(...$args)
    {}

    /**
     * Sets next count validator to the AtLeast instance
     *
     * @return self
     */
    public function atLeast()
    {}

    /**
     * Sets next count validator to the AtMost instance
     *
     * @return self
     */
    public function atMost()
    {}

    /**
     * Set the exception message
     *
     * @param  string $message
     * @return self
     */
    public function because($message)
    {}

    /**
     * Shorthand for setting minimum and maximum constraints on call counts
     *
     * @param  int  $minimum
     * @param  int  $maximum
     * @return self
     */
    public function between($minimum, $maximum)
    {}

    /**
     * Mark this expectation as being a default
     *
     * @return self
     */
    public function byDefault()
    {}

    /**
     * @return null|string
     */
    public function getExceptionMessage()
    {}

    /**
     * Return the parent mock of the expectation
     *
     * @return MockInterface
     */
    #[Override]
    public function getMock()
    {}

    public function getName()
    {}

    /**
     * Return order number
     *
     * @return int
     */
    #[Override]
    public function getOrderNumber()
    {}

    /**
     * Indicates call order should apply globally
     *
     * @return self
     */
    public function globally()
    {}

    /**
     * Check if there is a constraint on call count
     *
     * @return bool
     */
    public function isCallCountConstrained()
    {}

    /**
     * Checks if this expectation is eligible for additional calls
     *
     * @return bool
     */
    public function isEligible()
    {}

    /**
     * Check if passed arguments match an argument expectation
     *
     * @return bool
     */
    public function matchArgs(array $args)
    {}

    /**
     * Indicates that this expectation is never expected to be called
     *
     * @return self
     */
    public function never()
    {}

    /**
     * Indicates that this expectation is expected exactly once
     *
     * @return self
     */
    public function once()
    {}

    /**
     * Indicates that this expectation must be called in a specific given order
     *
     * @param  string $group Name of the ordered group
     * @return self
     */
    public function ordered($group = null)
    {}

    /**
     * Flag this expectation as calling the original class method with
     * the provided arguments instead of using a return value queue.
     *
     * @return self
     */
    public function passthru()
    {}

    /**
     * Alias to andSet(). Allows the natural English construct
     * - set('foo', 'bar')->andReturn('bar')
     *
     * @param  string $name
     * @param  mixed  $value
     * @return self
     */
    public function set($name, $value)
    { func_get_args(); }

    /**
     * Indicates the number of times this expectation should occur
     *
     * @param  int  $limit
     * @return self
     *
     * @throws InvalidArgumentException
     */
    public function times($limit = null)
    {}

    /**
     * Indicates that this expectation is expected exactly twice
     *
     * @return self
     */
    public function twice()
    {}

    /**
     * Verify call order
     *
     * @return void
     */
    public function validateOrder()
    {}

    /**
     * Verify this expectation
     *
     * @return void
     */
    public function verify()
    {}

    /**
     * Verify the current call, i.e. that the given arguments match those
     * of this expectation
     *
     * @param  array<mixed> $args
     * @return mixed
     *
     * @throws Throwable
     */
    public function verifyCall(array $args)
    {}

    /**
     * Expected argument setter for the expectation
     *
     * @param  mixed ...$args
     * @return self
     */
    public function with(...$args)
    {}

    /**
     * Set expectation that the given arguments are acceptable plus variable other arguments
     *
     * @param  mixed ...$args
     * @return self
     */
    public function withAndOthers(...$args)
    {}

    /**
     * Set expectation that any arguments are acceptable
     *
     * @return self
     */
    public function withAnyArgs()
    {}

    /**
     * Expected arguments for the expectation passed as an array or a closure that matches each passed argument on
     * each function call.
     *
     * @param  array|Closure $argsOrClosure
     * @return self
     */
    public function withArgs($argsOrClosure)
    {}

    /**
     * Set with() as no arguments expected
     *
     * @return self
     */
    public function withNoArgs()
    {}

    /**
     * Expected arguments should partially match the real arguments
     *
     * @param  mixed ...$expectedArgs
     * @return self
     */
    public function withSomeOfArgs(...$expectedArgs)
    {}

    /**
     * Indicates this expectation should occur zero or more times
     *
     * @return self
     */
    public function zeroOrMoreTimes()
    {}

    /**
     * Setup the ordering tracking on the mock or mock container
     *
     * @param  string                  $group
     * @param  Container|MockInterface $ordering
     * @return int
     */
    protected function _defineOrdered($group, $ordering)
    {}

    /**
     * Fetch the return value for the matching args
     *
     * @param  array<mixed> $args
     * @return mixed
     */
    protected function _getReturnValue(array $args)
    {}

    /**
     * Check if passed argument matches an argument expectation
     *
     * @param  mixed $expected
     * @param  mixed $actual
     * @return bool
     */
    protected function _matchArg($expected, &$actual)
    {}

    /**
     * Check if the passed arguments match the expectations, one by one.
     *
     * @param  array $args
     * @return bool
     */
    protected function _matchArgs($args)
    {}

    /**
     * Sets public properties with queued values to the mock object
     *
     * @return void
     */
    protected function _setValues()
    {}

    /**
     * @param  mixed $expectedArg
     * @return bool
     */
    private function isAndAnyOtherArgumentsMatcher($expectedArg)
    {}

    /**
     * Throws an exception if the expectation has been configured to do so
     *
     * @param  mixed|Throwable $return
     * @return never
     *
     * @throws Throwable
     */
    private function throwAsNecessary($return)
    {}

    /**
     * Expected arguments for the expectation passed as an array
     *
     * @param  array<mixed> $arguments
     * @return self
     */
    private function withArgsInArray(array $arguments)
    {}

    /**
     * Expected arguments have to be matched by the given closure.
     *
     * @return self
     */
    private function withArgsMatchedByClosure(Closure $closure)
    {}
}
