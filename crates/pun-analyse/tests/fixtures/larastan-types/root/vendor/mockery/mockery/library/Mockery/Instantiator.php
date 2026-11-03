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
use Exception;
use InvalidArgumentException;
use ReflectionClass;
use UnexpectedValueException;

use function class_exists;
use function restore_error_handler;
use function set_error_handler;
use function sprintf;
use function strlen;
use function unserialize;

/**
 * This is a trimmed down version of https://github.com/doctrine/instantiator, without the caching mechanism.
 */
final class Instantiator
{
    /**
     * @template TObject of object
     *
     * @param  class-string<TObject> $className
     * @return TObject
     *
     * @throws InvalidArgumentException
     * @throws UnexpectedValueException
     */
    public function instantiate($className): object
    {}

    /**
     * @throws UnexpectedValueException
     */
    private function attemptInstantiationViaUnSerialization(
        ReflectionClass $reflectionClass,
        string $serializedString
    ): void {}

    /**
     * Builds a {@see Closure} capable of instantiating the given $className without invoking its constructor.
     */
    private function buildFactory(string $className): Closure
    {}

    /**
     * @template TObject of object
     *
     * @param  class-string<TObject>    $className
     * @return ReflectionClass<TObject>
     *
     * @throws InvalidArgumentException
     */
    private function getReflectionClass(string $className): ReflectionClass
    {}

    /**
     * Verifies whether the given class is to be considered internal
     */
    private function hasInternalAncestors(ReflectionClass $reflectionClass): bool
    {}

    /**
     * Verifies if the class is instantiable via reflection
     */
    private function isInstantiableViaReflection(ReflectionClass $reflectionClass): bool
    {}
}
