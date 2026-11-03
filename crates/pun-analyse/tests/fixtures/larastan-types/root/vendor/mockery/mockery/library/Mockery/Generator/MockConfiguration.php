<?php

/**
 * Mockery (https://docs.mockery.io/en/stable/)
 *
 * @copyright https://github.com/mockery/mockery/blob/HEAD/COPYRIGHT.md
 * @license   https://github.com/mockery/mockery/blob/HEAD/LICENSE BSD 3-Clause License
 * @see       https://github.com/mockery/mockery for the canonical source repository
 */

namespace Mockery\Generator;

use Iterator;
use IteratorAggregate;
use Mockery\Exception;
use ReflectionClass;
use Serializable;

use function array_filter;
use function array_key_exists;
use function array_keys;
use function array_map;
use function array_merge;
use function array_pop;
use function array_unique;
use function array_values;
use function class_alias;
use function class_exists;
use function explode;
use function get_class;
use function implode;
use function in_array;
use function interface_exists;
use function is_object;
use function md5;
use function preg_match;
use function serialize;
use function strpos;
use function strtolower;
use function trait_exists;

/**
 * This class describes the configuration of mocks and hides away some of the
 * reflection implementation
 */
class MockConfiguration
{
    /**
     * Instance cache of all methods
     *
     * @var list<Method>
     */
    protected $allMethods = [];

    /**
     * Methods that should specifically not be mocked
     *
     * This is currently populated with stuff we don't know how to deal with, should really be somewhere else
     */
    protected $blackListedMethods = [];

    protected $constantsMap = [];

    /**
     * An instance mock is where we override the original class before it's autoloaded
     *
     * @var bool
     */
    protected $instanceMock = false;

    /**
     * If true, overrides original class destructor
     *
     * @var bool
     */
    protected $mockOriginalDestructor = false;

    /**
     * The class name we'd like to use for a generated mock
     *
     * @var string|null
     */
    protected $name;

    /**
     * Param overrides
     *
     * @var array<string,mixed>
     */
    protected $parameterOverrides = [];

    /**
     * A class that we'd like to mock
     *
     * @var TargetClassInterface|null
     */
    protected $targetClass;

    /**
     * @var class-string|null
     */
    protected $targetClassName;

    /**
     * @var list<class-string>
     */
    protected $targetInterfaceNames = [];

    /**
     * A number of interfaces we'd like to mock, keyed by name to attempt to keep unique
     *
     * @var list<TargetClassInterface>
     */
    protected $targetInterfaces = [];

    /**
     * An object we'd like our mock to proxy to
     *
     * @var object|null
     */
    protected $targetObject;

    /**
     * @var array<class-string>
     */
    protected $targetTraitNames = [];

    /**
     * A number of traits we'd like to mock, keyed by name to attempt to keep unique
     *
     * @var array<string,DefinedTargetClass>
     */
    protected $targetTraits = [];

    /**
     * If not empty, only these methods will be mocked
     *
     * @var array<string>
     */
    protected $whiteListedMethods = [];

    /**
     * @param array<class-string|object>         $targets
     * @param array<string>                      $blackListedMethods
     * @param array<string>                      $whiteListedMethods
     * @param string|null                        $name
     * @param bool                               $instanceMock
     * @param array<string,mixed>                $parameterOverrides
     * @param bool                               $mockOriginalDestructor
     * @param array<string,array<scalar>|scalar> $constantsMap
     */
    public function __construct(
        array $targets = [],
        array $blackListedMethods = [],
        array $whiteListedMethods = [],
        $name = null,
        $instanceMock = false,
        array $parameterOverrides = [],
        $mockOriginalDestructor = false,
        array $constantsMap = []
    ) {}

    /**
     * Generate a suitable name based on the config
     *
     * @return class-string
     *
     * @throws Exception
     */
    public function generateName()
    {}

    /**
     * @return array<string>
     */
    public function getBlackListedMethods()
    {}

    /**
     * @return array<string,scalar|array<scalar>>
     */
    public function getConstantsMap()
    {}

    /**
     * Attempt to create a hash of the configuration, in order to allow caching
     *
     * @TODO workout if this will work
     *
     * @return string
     */
    public function getHash()
    {}

    /**
     * Gets a list of methods from the classes, interfaces and objects and filters them appropriately.
     * Lot's of filtering going on, perhaps we could have filter classes to iterate through
     *
     * @return list<Method>
     *
     * @throws Exception
     */
    public function getMethodsToMock()
    {}

    /**
     * @return class-string|null
     */
    public function getName()
    {}

    /**
     * @return string
     */
    public function getNamespaceName()
    {}

    /**
     * @return array<string,mixed>
     */
    public function getParameterOverrides()
    {}

    /**
     * @return string
     */
    public function getShortName()
    {}

    /**
     * @return null|TargetClassInterface
     *
     * @throws Exception
     */
    public function getTargetClass()
    {}

    /**
     * @return class-string|null
     */
    public function getTargetClassName()
    {}

    /**
     * @return list<TargetClassInterface>
     */
    public function getTargetInterfaces()
    {}

    /**
     * @return object|null
     */
    public function getTargetObject()
    {}

    /**
     * @return list<TargetClassInterface>
     */
    public function getTargetTraits()
    {}

    /**
     * @return array<string>
     */
    public function getWhiteListedMethods()
    {}

    /**
     * @return bool
     */
    public function isInstanceMock()
    {}

    /**
     * @return bool
     */
    public function isMockOriginalDestructor()
    {}

    /**
     * @param  class-string $className
     * @return self
     */
    public function rename($className)
    {}

    /**
     * We declare the __callStatic method to handle undefined stuff, if the class
     * we're mocking has also defined it, we need to comply with their interface
     *
     * @return bool
     *
     * @throws Exception
     */
    public function requiresCallStaticTypeHintRemoval()
    {}

    /**
     * We declare the __call method to handle undefined stuff, if the class
     * we're mocking has also defined it, we need to comply with their interface
     *
     * @return bool
     *
     * @throws Exception
     */
    public function requiresCallTypeHintRemoval()
    {}

    /**
     * @param class-string|object $target
     */
    protected function addTarget($target)
    {}

    /**
     * If we attempt to implement Traversable,
     * we must ensure we are also implementing either Iterator or IteratorAggregate,
     * and that whichever one it is comes before Traversable in the list of implements.
     *
     * @param class-string $targetInterface
     */
    protected function addTargetInterfaceName($targetInterface)
    {}

    /**
     * @param class-string $targetTraitName
     */
    protected function addTargetTraitName($targetTraitName)
    {}

    /**
     * @param array<class-string> $interfaces
     */
    protected function addTargets($interfaces)
    {}

    /**
     * @return list<Method>
     *
     * @throws Exception
     */
    protected function getAllMethods()
    {}

    /**
     * @param class-string $targetClassName
     */
    protected function setTargetClassName($targetClassName)
    {}

    /**
     * @param object $object
     */
    protected function setTargetObject($object)
    {}
}
