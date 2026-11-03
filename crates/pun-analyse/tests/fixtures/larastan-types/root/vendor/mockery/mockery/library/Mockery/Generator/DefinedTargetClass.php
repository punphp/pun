<?php

/**
 * Mockery (https://docs.mockery.io/en/stable/)
 *
 * @copyright https://github.com/mockery/mockery/blob/HEAD/COPYRIGHT.md
 * @license   https://github.com/mockery/mockery/blob/HEAD/LICENSE BSD 3-Clause License
 * @see       https://github.com/mockery/mockery for the canonical source repository
 */

namespace Mockery\Generator;

use Override;
use ReflectionAttribute;
use ReflectionClass;
use ReflectionMethod;

use ReturnTypeWillChange;

use const PHP_VERSION_ID;

use function array_map;
use function array_merge;
use function array_unique;

class DefinedTargetClass implements TargetClassInterface
{
    /**
     * @var class-string
     */
    private $name;

    /**
     * @var ReflectionClass
     */
    private $reflectionClass;

    /**
     * @param class-string|null $alias
     */
    public function __construct(ReflectionClass $rfc, $alias = null)
    {}

    /**
     * @return class-string
     */
    #[ReturnTypeWillChange]
    public function __toString()
    {}

    /**
     * @param  class-string      $name
     * @param  class-string|null $alias
     * @return self
     */
    #[Override]
    public static function factory($name, $alias = null)
    {}

    /**
     * @return list<class-string>
     */
    #[Override]
    public function getAttributes()
    {}

    /**
     * @return array<class-string,self>
     */
    #[Override]
    public function getInterfaces()
    {}

    /**
     * @return list<Method>
     */
    #[Override]
    public function getMethods()
    {}

    /**
     * @return class-string
     */
    #[Override]
    public function getName()
    {}

    /**
     * @return string
     */
    #[Override]
    public function getNamespaceName()
    {}

    /**
     * @return string
     */
    #[Override]
    public function getShortName()
    {}

    /**
     * @return bool
     */
    #[Override]
    public function hasInternalAncestor()
    {}

    /**
     * @param  class-string $interface
     * @return bool
     */
    #[Override]
    public function implementsInterface($interface)
    {}

    /**
     * @return bool
     */
    #[Override]
    public function inNamespace()
    {}

    /**
     * @return bool
     */
    #[Override]
    public function isAbstract()
    {}

    /**
     * @return bool
     */
    #[Override]
    public function isFinal()
    {}

    /**
     * @return bool
     */
    #[Override]
    public function isReadOnly()
    {}
}
