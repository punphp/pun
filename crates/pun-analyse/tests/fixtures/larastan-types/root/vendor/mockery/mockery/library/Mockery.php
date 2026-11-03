<?php

/**
 * Mockery (https://docs.mockery.io/en/stable/)
 *
 * @copyright https://github.com/mockery/mockery/blob/HEAD/COPYRIGHT.md
 * @license   https://github.com/mockery/mockery/blob/HEAD/LICENSE BSD 3-Clause License
 * @see       https://github.com/mockery/mockery for the canonical source repository
 */

use Mockery\ClosureWrapper;
use Mockery\CompositeExpectation;
use Mockery\Configuration;
use Mockery\Container;
use Mockery\Exception as MockeryException;
use Mockery\Expectation;
use Mockery\ExpectationDirector;
use Mockery\ExpectationInterface;
use Mockery\Generator\CachingGenerator;
use Mockery\Generator\Generator;
use Mockery\Generator\MockConfigurationBuilder;
use Mockery\Generator\MockNameBuilder;
use Mockery\Generator\StringManipulationGenerator;
use Mockery\LegacyMockInterface;
use Mockery\Loader\EvalLoader;
use Mockery\Loader\Loader;
use Mockery\Matcher\AndAnyOtherArgs;
use Mockery\Matcher\Any;
use Mockery\Matcher\AnyOf;
use Mockery\Matcher\Closure as ClosureMatcher;
use Mockery\Matcher\Contains;
use Mockery\Matcher\Ducktype;
use Mockery\Matcher\HasKey;
use Mockery\Matcher\HasValue;
use Mockery\Matcher\IsEqual;
use Mockery\Matcher\IsSame;
use Mockery\Matcher\MatcherInterface;
use Mockery\Matcher\MustBe;
use Mockery\Matcher\Not;
use Mockery\Matcher\NotAnyOf;
use Mockery\Matcher\Pattern;
use Mockery\Matcher\Subset;
use Mockery\Matcher\Type;
use Mockery\MockInterface;
use Mockery\Reflector;

class Mockery
{
    public const BLOCKS = 'Mockery_Forward_Blocks';

    /**
     * Global configuration handler containing configuration options.
     *
     * @var null|Configuration
     */
    protected static $_config;

    /**
     * Global container to hold all mocks for the current unit test running.
     *
     * @var null|Container
     */
    protected static $_container;

    /**
     * @var null|Generator
     */
    protected static $_generator;

    /**
     * @var null|Loader
     */
    protected static $_loader;

    /**
     * @var list<string>
     */
    private static $_filesToCleanUp = [];

    /**
     * Return instance of AndAnyOtherArgs matcher.
     *
     * @return AndAnyOtherArgs
     */
    public static function andAnyOtherArgs()
    {}

    /**
     * Return instance of AndAnyOtherArgs matcher.
     *
     * An alternative name to `andAnyOtherArgs` so
     * the API stays closer to `any` as well.
     *
     * @return AndAnyOtherArgs
     */
    public static function andAnyOthers()
    {}

    /**
     * Return instance of ANY matcher.
     *
     * @return Any
     */
    public static function any()
    {}

    /**
     * Return instance of ANYOF matcher.
     *
     * @param  mixed ...$args
     * @return AnyOf
     */
    public static function anyOf(...$args)
    {}

    /**
     * @return array
     *
     * @deprecated since 1.3.2 and will be removed in 2.0.
     */
    public static function builtInTypes()
    {}

    /**
     * Return instance of CLOSURE matcher.
     *
     * @param  mixed          $reference
     * @return ClosureMatcher
     */
    public static function capture(&$reference)
    {}

    /**
     * Static shortcut to closing up and verifying all mocks in the global
     * container, and resetting the container static variable to null.
     *
     * @return void
     *
     * @throws Throwable
     */
    public static function close()
    {}

    /**
     * Return instance of CONTAINS matcher.
     *
     * @param  mixed    ...$args
     * @return Contains
     */
    public static function contains(...$args)
    {}

    /**
     * @param  class-string $fqn
     * @return void
     */
    public static function declareClass($fqn)
    {}

    /**
     * @param  class-string $fqn
     * @return void
     */
    public static function declareInterface($fqn)
    {}

    /**
     * Return instance of DUCKTYPE matcher.
     *
     * @param  mixed    ...$args
     * @return Ducktype
     */
    public static function ducktype(...$args)
    {}

    /**
     * Static fetching of a mock associated with a name or explicit class poser.
     *
     * @param  class-string       $name
     * @return null|MockInterface
     */
    public static function fetchMock($name)
    {}

    /**
     * Utility method to format method name and arguments into a string.
     *
     * @param  string $method
     * @return string
     */
    public static function formatArgs($method, ?array $arguments = null)
    {}

    /**
     * Utility function to format objects to printable arrays.
     *
     * @return string
     */
    public static function formatObjects(?array $objects = null)
    {}

    /**
     * Lazy loader and Getter for the global
     * configuration container.
     *
     * @return Configuration
     */
    public static function getConfiguration()
    {}

    /**
     * Lazy loader and getter for the container property.
     *
     * @return Container
     */
    public static function getContainer()
    {}

    /**
     * Creates and returns a default generator
     * used inside this class.
     *
     * @return CachingGenerator
     */
    public static function getDefaultGenerator()
    {}

    /**
     * Gets an EvalLoader to be used as default.
     *
     * @return EvalLoader
     */
    public static function getDefaultLoader()
    {}

    /**
     * Lazy loader method and getter for
     * the generator property.
     *
     * @return Generator
     */
    public static function getGenerator()
    {}

    /**
     * Lazy loader method and getter for
     * the $_loader property.
     *
     * @return Loader
     */
    public static function getLoader()
    {}

    /**
     * Defines the global helper functions
     *
     * @return void
     */
    public static function globalHelpers()
    {}

    /**
     * Return instance of HASKEY matcher.
     *
     * @param  mixed  $key
     * @return HasKey
     */
    public static function hasKey($key)
    {}

    /**
     * Return instance of HASVALUE matcher.
     *
     * @param  mixed    $val
     * @return HasValue
     */
    public static function hasValue($val)
    {}

    /**
     * Static and Semantic shortcut to Container::mock().
     *
     * @param  mixed         ...$args
     * @return MockInterface
     *
     * @throws Throwable
     */
    public static function instanceMock(...$args)
    {}

    /**
     * @param  string $type
     * @return bool
     *
     * @deprecated since 1.3.2 and will be removed in 2.0.
     */
    public static function isBuiltInType($type)
    {}

    /**
     * Return instance of IsEqual matcher.
     *
     * @param mixed $expected
     */
    public static function isEqual($expected): IsEqual
    {}

    /**
     * Return instance of IsSame matcher.
     *
     * @param mixed $expected
     */
    public static function isSame($expected): IsSame
    {}

    /**
     * Static shortcut to Container::mock().
     *
     * @param  mixed         ...$args
     * @return MockInterface
     *
     * @throws Throwable
     */
    public static function mock(...$args)
    {}

    /**
     * Return instance of MUSTBE matcher.
     *
     * @param  mixed  $expected
     * @return MustBe
     */
    public static function mustBe($expected)
    {}

    /**
     * Static shortcut to Container::mock(), first argument names the mock.
     *
     * @param  mixed         ...$args
     * @return MockInterface
     *
     * @throws Throwable
     */
    public static function namedMock(...$args)
    {}

    /**
     * Return instance of NOT matcher.
     *
     * @param  mixed $expected
     * @return Not
     */
    public static function not($expected)
    {}

    /**
     * Return instance of NOTANYOF matcher.
     *
     * @param  mixed    ...$args
     * @return NotAnyOf
     */
    public static function notAnyOf(...$args)
    {}

    /**
     * Return instance of CLOSURE matcher.
     *
     * @param  Closure        $closure
     * @return ClosureMatcher
     */
    public static function on($closure)
    {}

    /**
     * Utility function to parse shouldReceive() arguments and generate
     * expectations from such as needed.
     *
     * @param  MockInterface                                         $mock
     * @param  mixed                                                 ...$args
     * @param  Closure(string):ExpectationInterface                  $add
     * @return CompositeExpectation|Expectation|ExpectationInterface
     */
    public static function parseShouldReturnArgs(LegacyMockInterface $mock, $args, $add)
    {}

    /**
     * Return instance of PATTERN matcher.
     *
     * @param  mixed   $expected
     * @return Pattern
     */
    public static function pattern($expected)
    {}

    /**
     * Register a file to be deleted on tearDown.
     *
     * @param string $fileName
     */
    public static function registerFileForCleanUp($fileName)
    {}

    /**
     * Reset the container to null.
     *
     * @return void
     */
    public static function resetContainer()
    {}

    /**
     * Static shortcut to Container::self().
     *
     * @return MockInterface
     *
     * @throws LogicException
     */
    public static function self()
    {}

    /**
     * Set the container.
     *
     * @return Container
     */
    public static function setContainer(Container $container)
    {}

    /**
     * Setter for the $_generator static property.
     */
    public static function setGenerator(Generator $generator)
    {}

    /**
     * Setter for the $_loader static property.
     */
    public static function setLoader(Loader $loader)
    {}

    /**
     * Static and semantic shortcut for getting a mock from the container
     * and applying the spy's expected behavior into it.
     *
     * @param  mixed         ...$args
     * @return MockInterface
     *
     * @throws Throwable
     */
    public static function spy(...$args)
    {}

    /**
     * Return instance of SUBSET matcher.
     *
     * @param  bool   $strict - (Optional) True for strict comparison, false for loose
     * @return Subset
     */
    public static function subset(array $part, $strict = true)
    {}

    /**
     * Return instance of TYPE matcher.
     *
     * @param  mixed $expected
     * @return Type
     */
    public static function type($expected)
    {}

    /**
     * Sets up expectations on the members of the CompositeExpectation and
     * builds up any demeter chain that was passed to shouldReceive.
     *
     * @param  MockInterface                        $mock
     * @param  string                               $arg
     * @param  Closure(string):ExpectationInterface $add
     * @return ExpectationInterface
     *
     * @throws Throwable
     */
    protected static function buildDemeterChain($mock, $arg, $add)
    {}

    /**
     * @param MockInterface $mock
     * @param list<string>  $methodNames
     *
     * @throws Throwable
     */
    private static function assertValidDemeterChain(LegacyMockInterface $mock, array $methodNames): void
    {}

    /**
     * Utility method for recursively generating a representation of the given array.
     *
     * @return '...'|array
     */
    private static function cleanupArray(array $argument, int $nesting = 3)
    {}

    /**
     * Utility method used for recursively generating
     * an object or array representation.
     *
     * @param  mixed $argument
     * @param  int   $nesting
     * @return mixed
     */
    private static function cleanupNesting($argument, $nesting)
    {}

    /**
     * @param class-string $fqn
     * @param string       $type
     */
    private static function declareType($fqn, $type): void
    {}

    /**
     * Returns all public instance properties.
     *
     * @param  object               $object
     * @param  int                  $nesting
     * @return array<string, mixed>
     */
    private static function extractInstancePublicProperties($object, $nesting)
    {}

    /**
     * Gets the string representation
     * of any passed argument.
     *
     * @param  mixed $argument
     * @param  int   $depth
     * @return mixed
     */
    private static function formatArgument($argument, $depth = 0)
    {}

    /**
     * Gets a specific demeter mock from the ones kept by the container.
     *
     * @param  class-string       $demeterMockKey
     * @return null|MockInterface
     */
    private static function getExistingDemeterMock(Container $container, $demeterMockKey)
    {}

    /**
     * Gets a new demeter configured
     * mock from the container.
     *
     * @param  string        $parent
     * @param  string        $method
     * @return MockInterface
     *
     * @throws Throwable
     */
    private static function getNewDemeterMock(Container $container, $parent, $method, ExpectationInterface $expectation)
    {}

    /**
     * Utility function to turn public properties and public get* and is* method values into an array.
     *
     * @param  object $object
     * @param  int    $nesting
     * @return array
     */
    private static function objectToArray($object, $nesting = 3)
    {}
}
