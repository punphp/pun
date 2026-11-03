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

use ReturnTypeWillChange;

use function array_pop;
use function explode;
use function implode;
use function ltrim;

class UndefinedTargetClass implements TargetClassInterface
{
    /**
     * @var class-string
     */
    private $name;

    /**
     * @param class-string $name
     */
    public function __construct($name)
    {}

    /**
     * @return class-string
     */
    #[ReturnTypeWillChange]
    public function __toString()
    {}

    /**
     * @param  class-string $name
     * @return self
     */
    #[Override]
    public static function factory($name)
    {}

    /**
     * @return list<class-string>
     */
    #[Override]
    public function getAttributes()
    {}

    /**
     * @return list<self>
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
