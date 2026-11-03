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
use Exception as PHPException;
use LogicException;
use Mockery;
use Mockery\Exception\BadMethodCallException;
use Mockery\Exception\InvalidOrderException;
use Mockery\Exception\RuntimeException;
use Mockery\Generator\Generator;
use Mockery\Generator\MockConfiguration;
use Mockery\Generator\MockConfigurationBuilder;
use Mockery\Loader\Loader as LoaderInterface;
use ReflectionClass;
use stdClass;
use Throwable;

use function array_filter;
use function array_key_exists;
use function array_keys;
use function array_pop;
use function array_shift;
use function array_values;
use function class_exists;
use function count;
use function explode;
use function get_class;
use function interface_exists;
use function is_array;
use function is_object;
use function is_string;
use function md5;
use function preg_grep;
use function preg_match;
use function range;
use function reset;
use function rtrim;
use function sprintf;
use function str_contains;
use function str_ends_with;
use function str_replace;
use function str_starts_with;
use function strtolower;
use function substr;
use function trait_exists;
use function trim;

/**
 * Container for mock objects
 */
class Container
{
    public const BLOCKS = Mockery::BLOCKS;

    /**
     * Order number of allocation
     *
     * @var int
     */
    protected $_allocatedOrder = 0;

    /**
     * Current ordered number
     *
     * @var int
     */
    protected $_currentOrder = 0;

    /**
     * @var Generator
     */
    protected $_generator;

    /**
     * Ordered groups
     *
     * @var array<string,int>
     */
    protected $_groups = [];

    /**
     * @var LoaderInterface
     */
    protected $_loader;

    /**
     * Store of mock objects
     *
     * @var array<class-string<MockInterface>,MockInterface>
     */
    protected $_mocks = [];

    /**
     * @var array<string,string>
     */
    protected $_namedMocks = [];

    /**
     * @var Instantiator
     */
    protected $instantiator;

    public function __construct(?Generator $generator = null, ?LoaderInterface $loader = null, ?Instantiator $instantiator = null)
    {}

    /**
     * Return a specific remembered mock according to the array index it
     * was stored to in this container instance
     *
     * @param  class-string       $reference
     * @return null|MockInterface
     */
    public function fetchMock($reference)
    {}

    /**
     * @return Generator
     */
    public function getGenerator()
    {}

    /**
     * @param  string      $method
     * @param  string      $parent
     * @return null|string
     */
    public function getKeyOfDemeterMockFor($method, $parent)
    {}

    /**
     * @return LoaderInterface
     */
    public function getLoader()
    {}

    /**
     * @return array<class-string<MockInterface>,MockInterface>
     */
    public function getMocks()
    {}

    /**
     * @return void
     */
    public function instanceMock() {}

    /**
     * see http://php.net/manual/en/language.oop5.basic.php
     *
     * @param  string $className
     * @return bool
     */
    public function isValidClassName($className)
    {}

    /**
     * Generates a new mock object for this container
     *
     * I apologies in advance for this. A God Method just fits the API which
     * doesn't require differentiating between classes, interfaces, abstracts,
     * names or partials - just so long as it's something that can be mocked.
     * I'll refactor it one day so it's easier to follow.
     *
     * @param  mixed         ...$args
     * @return MockInterface
     *
     * @throws Throwable
     */
    public function mock(...$args)
    {}

    /**
     * Fetch the next available allocation order number
     *
     * @return int
     */
    public function mockery_allocateOrder()
    {}

    /**
     * Reset the container to its original state
     *
     * @return void
     */
    public function mockery_close()
    {}

    /**
     * Get current ordered number
     *
     * @return int
     */
    public function mockery_getCurrentOrder()
    {}

    /**
     * Gets the count of expectations on the mocks
     *
     * @return int
     */
    public function mockery_getExpectationCount()
    {}

    /**
     * Fetch array of ordered groups
     *
     * @return array<string,int>
     */
    public function mockery_getGroups()
    {}

    /**
     * Set current ordered number
     *
     * @param  int $order
     * @return int The current order number that was set
     */
    public function mockery_setCurrentOrder($order)
    {}

    /**
     * Set ordering for a group
     *
     * @param  string $group
     * @param  int    $order
     * @return void
     */
    public function mockery_setGroup($group, $order)
    {}

    /**
     * Tear down tasks for this container
     *
     * @return void
     *
     * @throws PHPException
     */
    public function mockery_teardown()
    {}

    /**
     * Retrieves all exceptions thrown by mocks
     *
     * @return array<BadMethodCallException>
     */
    public function mockery_thrownExceptions()
    {}

    /**
     * Validate the current mock's ordering
     *
     * @param  string $method
     * @param  int    $order
     * @return void
     *
     * @throws Exception
     */
    public function mockery_validateOrder($method, $order, LegacyMockInterface $mock)
    {}

    /**
     * Verify the container mocks
     *
     * @return void
     */
    public function mockery_verify()
    {}

    /**
     * Store a mock and set its container reference
     *
     * @param  MockInterface $mock
     * @return MockInterface
     */
    public function rememberMock(LegacyMockInterface $mock)
    {}

    /**
     * Retrieve the last remembered mock object,
     * which is the same as saying retrieve the current mock being programmed where you have yet to call mock()
     * to change it thus why the method name is "self" since it will be used during the programming of the same mock.
     *
     * @return MockInterface
     *
     * @throws LogicException
     */
    public function self()
    {}

    /**
     * @template TObject of object
     *
     * @param  class-string<TObject> $mockName
     * @param  null|list<mixed>      $constructorArgs
     * @return TObject
     *
     * @throws Throwable
     */
    protected function _getInstance($mockName, $constructorArgs = null)
    {}

    /**
     * @param  MockConfiguration $config
     * @return void
     *
     * @throws Throwable
     */
    protected function checkForNamedMockClashes($config)
    {}

    private function createBuilder(array &$arguments): MockConfigurationBuilder
    {}

    /**
     * @return class-string
     *
     * @throws Throwable
     */
    private function generateMock(MockConfiguration $mockConfiguration): string
    {}

    /**
     * @return null|Closure(MockInterface):void
     */
    private function handleClosure(array &$arguments): ?Closure
    {}

    private function initializeBuilder(array &$arguments): MockConfigurationBuilder
    {}

    /**
     * @param class-string     $className
     * @param null|list<mixed> $constructorArgs
     */
    private function initializeMock(
        string $className,
        ?array $constructorArgs,
        MockConfiguration $mockConfiguration
    ): object {}

    private function parseArguments(array &$arguments): array
    {}

    private function parseArrayArgument(
        array $argument,
        array &$quickDefinitions,
        ?array &$constructorArgs,
        array &$blocks
    ): void {}

    private function parseStringArgument(
        string $arguments,
        MockConfigurationBuilder $builder,
        ?array &$partialMethods
    ): void {}
}
