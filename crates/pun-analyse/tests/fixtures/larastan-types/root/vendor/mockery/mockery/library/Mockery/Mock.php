<?php

/**
 * Mockery (https://docs.mockery.io/en/stable/)
 *
 * @copyright https://github.com/mockery/mockery/blob/HEAD/COPYRIGHT.md
 * @license   https://github.com/mockery/mockery/blob/HEAD/LICENSE BSD 3-Clause License
 * @see       https://github.com/mockery/mockery for the canonical source repository
 */

namespace Mockery;

use Mockery\Container;
use Mockery\CountValidator\Exception;
use Mockery\Exception\BadMethodCallException;
use Mockery\Exception\InvalidOrderException;
use Mockery\Exception\NoMatchingExpectationException;
use Mockery\Expectation;
use Mockery\ExpectationDirector;
use Mockery\ExpectsHigherOrderMessage;
use Mockery\HigherOrderMessage;
use Mockery\LegacyMockInterface;
use Mockery\MethodCall;
use Mockery\MockInterface;
use Mockery\ReceivedMethodCalls;
use Mockery\Reflector;
use Mockery\Undefined;
use Mockery\VerificationDirector;
use Mockery\VerificationExpectation;

#[\AllowDynamicProperties]
class Mock implements MockInterface
{
    /**
     * Stores an array of all expectation directors for this mock
     *
     * @var array<string,ExpectationDirector>
     */
    protected $_mockery_expectations = [];

    /**
     * Stores an initial number of expectations that can be manipulated
     * while using the getter method.
     *
     * @var int
     */
    protected $_mockery_expectations_count = 0;

    /**
     * Flag to indicate whether we can ignore method calls missing from our
     * expectations
     *
     * @var bool
     */
    protected $_mockery_ignoreMissing = false;

    /**
     * Flag to indicate whether we want to set the ignoreMissing flag on
     * mocks generated form this calls to this one
     *
     * @var bool
     */
    protected $_mockery_ignoreMissingRecursive = false;

    /**
     * Flag to indicate whether we can defer method calls missing from our
     * expectations
     *
     * @var bool
     */
    protected $_mockery_deferMissing = false;

    /**
     * Flag to indicate whether this mock was verified
     *
     * @var bool
     */
    protected $_mockery_verified = false;

    /**
     * Given name of the mock
     *
     * @var string
     */
    protected $_mockery_name = null;

    /**
     * Order number of allocation
     *
     * @var int
     */
    protected $_mockery_allocatedOrder = 0;

    /**
     * Current ordered number
     *
     * @var int
     */
    protected $_mockery_currentOrder = 0;

    /**
     * Ordered groups
     *
     * @var array<string,int>
     */
    protected $_mockery_groups = [];

    /**
     * Mock container containing this mock object
     *
     * @var Container
     */
    protected $_mockery_container = null;

    /**
     * Instance of a core object on which methods are called in the event
     * it has been set, and an expectation for one of the object's methods
     * does not exist. This implements a simple partial mock proxy system.
     *
     * @var object
     */
    protected $_mockery_partial = null;

    /**
     * Flag to indicate we should ignore all expectations temporarily. Used
     * mainly to prevent expectation matching when in the middle of a mock
     * object recording session.
     *
     * @var bool
     */
    protected $_mockery_disableExpectationMatching = false;

    /**
     * Stores all stubbed public methods separate from any on-object public
     * properties that may exist.
     *
     * @var array<string,mixed>
     */
    protected $_mockery_mockableProperties = [];

    /**
     * @var list<string>
     */
    protected $_mockery_mockableMethods = [];

    /**
     * Just a local cache for this mock's target's methods
     *
     * @var \ReflectionMethod[]
     */
    protected static $_mockery_methods;

    protected $_mockery_allowMockingProtectedMethods = false;

    protected $_mockery_receivedMethodCalls;

    /**
     * If shouldIgnoreMissing is called, this value will be returned on all calls to missing methods
     * @var mixed
     */
    protected $_mockery_defaultReturnValue = null;

    /**
     * Tracks internally all the bad method call exceptions that happened during runtime
     *
     * @var array<BadMethodCallException>
     */
    protected $_mockery_thrownExceptions = [];

    protected $_mockery_instanceMock = true;

    /** @var null|class-string $parentClass */
    private $_mockery_parentClass = null;

    /**
     * We want to avoid constructors since class is copied to Generator.php
     * for inclusion on extending class definitions.
     *
     * @param Container $container
     * @param object|null $partialObject
     * @param bool $instanceMock
     * @return void
     */
    #[\Override]
    public function mockery_init(?Container $container = null, $partialObject = null, $instanceMock = true)
    {}

    /**
     * Set expected method calls
     *
     * @param string|array<string,mixed> ...$methodNames one or many methods that are expected to be called in this mock
     *
     * @return ($methodNames is list{} ? HigherOrderMessage : Expectation)
     */
    #[\Override]
    public function shouldReceive(...$methodNames)
    {}

    // start method allows
    /**
     * @param mixed $something  String method name or map of method => return
     *
     * @return ($something is string ? Expectation : ($something is list{} ? HigherOrderMessage : static))
     */
    public function allows($something = [])
    {}

    // end method allows
    // start method expects
    /**
    * @param mixed $something  String method name (optional)
    *
    * @return ($something is string ? Expectation : ExpectsHigherOrderMessage)
    */
    public function expects($something = null)
    {}

    // end method expects
    /**
     * Shortcut method for setting an expectation that a method should not be called.
     *
     * @param string|array<string,mixed> ...$methodNames one or many methods that are expected not to be called in this mock
     *
     * @return ($methodNames is list{} ? HigherOrderMessage : Expectation)
     */
    #[\Override]
    public function shouldNotReceive(...$methodNames)
    {}

    /**
     * Allows additional methods to be mocked that do not explicitly exist on mocked class
     *
     * @param string $method name of the method to be mocked
     *
     * @return static
     */
    #[\Override]
    public function shouldAllowMockingMethod($method)
    {}

    /**
     * Set mock to ignore unexpected methods and return Undefined class
     * @param mixed $returnValue the default return value for calls to missing functions on this mock
     * @param bool $recursive Specify if returned mocks should also have shouldIgnoreMissing set
     * @return static
     */
    #[\Override]
    public function shouldIgnoreMissing($returnValue = null, $recursive = false)
    {}

    /**
     * @return static
     */
    public function asUndefined()
    {}

    /**
     * @return static
     */
    #[\Override]
    public function shouldAllowMockingProtectedMethods()
    {}


    /**
     * Set mock to defer unexpected methods to it's parent
     *
     * This is particularly useless for this class, as it doesn't have a parent,
     * but included for completeness
     *
     * @deprecated 2.0.0 Please use makePartial() instead
     *
     * @return static
     */
    #[\Override]
    public function shouldDeferMissing()
    {}

    /**
     * Set mock to defer unexpected methods to it's parent
     *
     * It was an alias for shouldDeferMissing(), which will be removed
     * in 2.0.0.
     *
     * @return static
     */
    #[\Override]
    public function makePartial()
    {}

    /**
     * In the event shouldReceive() accepting one or more methods/returns,
     * this method will switch them from normal expectations to default
     * expectations
     *
     * @return static
     */
    #[\Override]
    public function byDefault()
    {}

    /**
     * Capture calls to this mock
     * 
     * @param string $method
     * @param array<mixed> $args
     * @return mixed
     * 
     * @throws \Throwable
     */
    public function __call($method, array $args)
    {}

    public static function __callStatic($method, array $args)
    {}

    /**
     * Forward calls to this magic method to the __call method
     * 
     * @throws \Throwable
     *
     * @return string
     */
    #[\ReturnTypeWillChange]
    public function __toString()
    {}

    /**
     * Iterate across all expectation directors and validate each
     *
     * @throws Exception
     * @return void
     */
    #[\Override]
    public function mockery_verify()
    {}

    /**
     * Gets a list of exceptions thrown by this mock
     *
     * @return array<BadMethodCallException>
     */
    public function mockery_thrownExceptions()
    {}

    /**
     * Tear down tasks for this mock
     *
     * @return void
     */
    #[\Override]
    public function mockery_teardown()
    {}

    /**
     * Fetch the next available allocation order number
     *
     * @return int
     */
    #[\Override]
    public function mockery_allocateOrder()
    {}

    /**
     * Set ordering for a group
     *
     * @param string $group
     * @param int $order
     */
    #[\Override]
    public function mockery_setGroup($group, $order)
    {}

    /**
     * Fetch array of ordered groups
     *
     * @return array<string,int>
     */
    #[\Override]
    public function mockery_getGroups()
    {}

    /**
     * Set current ordered number
     *
     * @param int $order
     * @return void
     */
    #[\Override]
    public function mockery_setCurrentOrder($order)
    {}

    /**
     * Get current ordered number
     *
     * @return int
     */
    #[\Override]
    public function mockery_getCurrentOrder()
    {}

    /**
     * Validate the current mock's ordering
     *
     * @param string $method
     * @param int $order
     * @throws \Mockery\Exception
     * @return void
     */
    #[\Override]
    public function mockery_validateOrder($method, $order)
    {}

    /**
     * Gets the count of expectations for this mock
     *
     * @return int
     */
    #[\Override]
    public function mockery_getExpectationCount()
    {}

    /**
     * Return the expectations director for the given method
     *
     * @var string $method
     * @return void
     */
    #[\Override]
    public function mockery_setExpectationsFor($method, ExpectationDirector $director)
    {}

    /**
     * Return the expectations director for the given method
     *
     * @var string $method
     * @return ExpectationDirector|null
     */
    #[\Override]
    public function mockery_getExpectationsFor($method)
    {}

    /**
     * Find an expectation matching the given method and arguments
     *
     * @var string $method
     * @var array $args
     * @return Expectation|null
     */
    #[\Override]
    public function mockery_findExpectation($method, array $args)
    {}

    /**
     * Return the container for this mock
     *
     * @return Container
     */
    #[\Override]
    public function mockery_getContainer()
    {}

    /**
     * Return the name for this mock
     *
     * @return string
     */
    #[\Override]
    public function mockery_getName()
    {}

    /**
     * @return array<string,mixed>
     */
    #[\Override]
    public function mockery_getMockableProperties()
    {}

    /**
     * @param string $name
     *
     * @return bool
     */
    public function __isset($name)
    {}

    /**
     * @return array<string,ExpectationDirector>
     */
    public function mockery_getExpectations()
    {}

    /**
     * Calls a parent class method and returns the result. Used in a passthru
     * expectation where a real return value is required while still taking
     * advantage of expectation matching and call count verification.
     *
     * @param string $name
     * @param array $args
     * @return mixed
     */
    public function mockery_callSubjectMethod($name, array $args)
    {}

    /**
     * @return list<string>
     */
    #[\Override]
    public function mockery_getMockableMethods()
    {}

    /**
     * @return bool
     */
    #[\Override]
    public function mockery_isAnonymous()
    {}

    public function mockery_isInstance()
    {}

    public function __wakeup()
    {}

    public function __destruct()
    {}

    /**
     * @param string $name Method name.
     *
     * @return \ReflectionMethod|null
     */
    public function mockery_getMethod($name)
    {}

    /**
     * @param string $name Method name.
     *
     * @return mixed Generated return value based on the declared return value of the named method.
     */
    public function mockery_returnValueForMethod($name)
    {}

    /**
     * @return ($method is null ? HigherOrderMessage : VerificationDirector)
     */
    #[\Override]
    public function shouldHaveReceived($method = null, $args = null)
    {}

    /**
     * @return VerificationDirector
     */
    #[\Override]
    public function shouldHaveBeenCalled()
    {}

    /**
     * @param null|string               $method
     * @param null|array<mixed>|\Closure $args
     *
     * @return ($method is null ? HigherOrderMessage : void)
     */
    #[\Override]
    public function shouldNotHaveReceived($method = null, $args = null)
    {}

    /**
     * @param null|array<mixed> $args (optional)
     *
     * @return void
     */
    #[\Override]
    public function shouldNotHaveBeenCalled(?array $args = null)
    {}

    protected static function _mockery_handleStaticMethodCall($method, array $args)
    {}

    protected function _mockery_getReceivedMethodCalls()
    {}

    /**
     * Called when an instance Mock was created and its constructor is getting called
     *
     * @see \Mockery\Generator\StringManipulation\Pass\InstanceMockPass
     * @param array $args
     * @throws \Throwable
     * @return void
     */
    protected function _mockery_constructorCalled(array $args)
    {}

    /**
     * @param string $method
     * @return ExpectationDirector|null
     */
    protected function _mockery_findExpectedMethodHandler($method)
    {}

    /**
     * @param string $method
     * @param array<mixed> $args
     *
     * @return mixed
     * 
     * @throws \Throwable
     */
    protected function _mockery_handleMethodCall($method, array $args)
    {}

    /**
     * Uses reflection to get the list of all
     * methods within the current mock object
     *
     * @return list<\ReflectionMethod>
     */
    protected function mockery_getMethods()
    {}

    private function hasMethodOverloadingInParentClass(): bool
    {}

    /**
     * @return list<string>
     */
    private function getNonPublicMethods(): array
    {}
}
