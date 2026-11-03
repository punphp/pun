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
use LogicException;
use Mockery\Matcher\MatcherInterface;

use const E_USER_DEPRECATED;
use const PHP_MAJOR_VERSION;

use function array_key_exists;
use function array_merge;
use function class_implements;
use function get_parent_class;
use function is_a;
use function sprintf;
use function strtolower;
use function trigger_error;

class Configuration
{
    /**
     * Boolean assertion of whether we ignore unnecessary mocking of methods,
     * i.e. when method expectations are made, set using a zeroOrMoreTimes()
     * constraint, and then never called. Essentially such expectations are
     * not required and are just taking up test space.
     *
     * @var bool
     */
    protected $_allowMockingMethodsUnnecessarily = true;

    /**
     * Boolean assertion of whether we can mock methods which do not actually
     * exist for the given class or object (ignored for unreal mocks)
     *
     * @var bool
     */
    protected $_allowMockingNonExistentMethod = true;

    /**
     * Constants map
     *
     * e.g. ['class' => ['MY_CONST' => 123, 'OTHER_CONST' => 'foo']]
     *
     * @var array<class-string,array<string,array<scalar>|scalar>>
     */
    protected $_constantsMap = [];

    /**
     * Default argument matchers
     *
     * e.g. ['class' => 'matcher']
     *
     * @var array<class-string,class-string>
     */
    protected $_defaultMatchers = [];

    /**
     * Parameter map for use with PHP internal classes.
     *
     *  e.g. ['class' => ['method' => ['param1', 'param2']]]
     *
     * @var array<class-string,array<string,list<string>>>
     */
    protected $_internalClassParamMap = [];

    /**
     * Custom object formatters
     *
     * e.g. ['class' => static fn($object) => 'formatted']
     *
     * @var array<class-string,Closure>
     */
    protected $_objectFormatters = [];

    /**
     * @var QuickDefinitionsConfiguration
     */
    protected $_quickDefinitionsConfiguration;

    /**
     * Boolean assertion is reflection caching enabled or not. It should be
     * always enabled, except when using PHPUnit's --static-backup option.
     *
     * @see https://github.com/mockery/mockery/issues/268
     */
    protected $_reflectionCacheEnabled = true;

    public function __construct()
    {}

    /**
     * Set boolean to allow/prevent unnecessary mocking of methods
     *
     * @param  bool $flag
     * @return void
     *
     * @deprecated since 1.4.0
     */
    public function allowMockingMethodsUnnecessarily($flag = true)
    {}

    /**
     * Set boolean to allow/prevent mocking of non-existent methods
     *
     * @param  bool $flag
     * @return void
     */
    public function allowMockingNonExistentMethods($flag = true)
    {}

    /**
     * Disable reflection caching
     *
     * It should be always enabled, except when using
     * PHPUnit's --static-backup option.
     *
     * @see https://github.com/mockery/mockery/issues/268
     *
     * @return void
     */
    public function disableReflectionCache()
    {}

    /**
     * Enable reflection caching
     *
     * It should be always enabled, except when using
     * PHPUnit's --static-backup option.
     *
     * @see https://github.com/mockery/mockery/issues/268
     *
     * @return void
     */
    public function enableReflectionCache()
    {}

    /**
     * Get the map of constants to be used in the mock generator
     *
     * @return array<class-string,array<string,array<scalar>|scalar>>
     */
    public function getConstantsMap()
    {}

    /**
     * Get the default matcher for a given class
     *
     * @param  class-string      $class
     * @return null|class-string
     */
    public function getDefaultMatcher($class)
    {}

    /**
     * Get the parameter map of an internal PHP class method
     *
     * @param  class-string $class
     * @param  string       $method
     * @return null|array
     */
    public function getInternalClassMethodParamMap($class, $method)
    {}

    /**
     * Get the parameter maps of internal PHP classes
     *
     * @return array<class-string,array<string,list<string>>>
     */
    public function getInternalClassMethodParamMaps()
    {}

    /**
     * Get the object formatter for a class
     *
     * @param  class-string $class
     * @param  Closure      $defaultFormatter
     * @return Closure
     */
    public function getObjectFormatter($class, $defaultFormatter)
    {}

    /**
     * Returns the quick definitions configuration
     */
    public function getQuickDefinitions(): QuickDefinitionsConfiguration
    {}

    /**
     * Return flag indicating whether mocking non-existent methods allowed
     *
     * @return bool
     *
     * @deprecated since 1.4.0
     */
    public function mockingMethodsUnnecessarilyAllowed()
    {}

    /**
     * Return flag indicating whether mocking non-existent methods allowed
     *
     * @return bool
     */
    public function mockingNonExistentMethodsAllowed()
    {}

    /**
     * Is reflection cache enabled?
     *
     * @return bool
     */
    public function reflectionCacheEnabled()
    {}

    /**
     * Remove all overridden parameter maps from internal PHP classes.
     *
     * @return void
     */
    public function resetInternalClassMethodParamMaps()
    {}

    /**
     * Set a map of constants to be used in the mock generator
     *
     * e.g. ['MyClass' => ['MY_CONST' => 123, 'ARRAY_CONST' => ['foo', 'bar']]]
     *
     * @param  array<class-string,array<string,array<scalar>|scalar>> $map
     * @return void
     */
    public function setConstantsMap(array $map)
    {}

    /**
     * @param  class-string $class
     * @param  class-string $matcherClass
     * @return void
     *
     * @throws InvalidArgumentException
     */
    public function setDefaultMatcher($class, $matcherClass)
    {}

    /**
     * Set a parameter map (array of param signature strings) for the method of an internal PHP class.
     *
     * @param  class-string $class
     * @param  string       $method
     * @param  list<string> $map
     * @return void
     *
     * @throws LogicException
     */
    public function setInternalClassMethodParamMap($class, $method, array $map)
    {}

    /**
     * Set a custom object formatter for a class
     *
     * @param  class-string $class
     * @param  Closure      $formatterCallback
     * @return void
     */
    public function setObjectFormatter($class, $formatterCallback)
    {}
}
