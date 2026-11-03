<?php

/**
 * Mockery (https://docs.mockery.io/en/stable/)
 *
 * @copyright https://github.com/mockery/mockery/blob/HEAD/COPYRIGHT.md
 * @license   https://github.com/mockery/mockery/blob/HEAD/LICENSE BSD 3-Clause License
 * @see       https://github.com/mockery/mockery for the canonical source repository
 */

namespace Mockery;

use InvalidArgumentException;
use ReflectionClass;
use ReflectionIntersectionType;
use ReflectionMethod;
use ReflectionNamedType;
use ReflectionParameter;
use ReflectionType;
use ReflectionUnionType;

use const PHP_VERSION_ID;

use function array_diff;
use function array_intersect;
use function array_map;
use function array_merge;
use function get_debug_type;
use function implode;
use function in_array;
use function method_exists;
use function sprintf;
use function strpos;
use function strtolower;

class Reflector
{
    /**
     * List of built-in types.
     *
     * @var list<string>
     */
    public const BUILTIN_TYPES = ['array', 'bool', 'int', 'float', 'null', 'object', 'string'];

    /**
     * List of reserved words.
     *
     * @var list<string>
     */
    public const RESERVED_WORDS = [
        'bool',
        'true',
        'false',
        'float',
        'int',
        'iterable',
        'mixed',
        'never',
        'null',
        'object',
        'string',
        'void'
    ];

    /**
     * Iterable.
     *
     * @var list<string>
     */
    private const ITERABLE = ['iterable'];

    /**
     * Traversable array.
     *
     * @var list<string>
     */
    private const TRAVERSABLE_ARRAY = ['\Traversable', 'array'];

    /**
     * Compute the string representation for the return type.
     *
     * @param  bool        $withoutNullable
     * @return null|string
     *
     * @throws InvalidArgumentException
     */
    public static function getReturnType(ReflectionMethod $method, $withoutNullable = false)
    {}

    /**
     * Compute the string representation for the simplest return type.
     *
     * @return null|string
     */
    public static function getSimplestReturnType(ReflectionMethod $method)
    {}

    /**
     * Compute the string representation for the paramater type.
     *
     * @param  bool        $withoutNullable
     * @return null|string
     *
     * @throws InvalidArgumentException
     */
    public static function getTypeHint(ReflectionParameter $param, $withoutNullable = false)
    {}

    /**
     * Determine if the parameter is typed as an array.
     *
     * @return bool
     */
    public static function isArray(ReflectionParameter $param)
    {}

    /**
     * Determine if the given type is a reserved word.
     */
    public static function isReservedWord(string $type): bool
    {}

    /**
     * Format the given type as a nullable type.
     */
    private static function formatNullableType(string $typeHint): string
    {}

    /**
     * @throws InvalidArgumentException
     */
    private static function getTypeFromReflectionType(ReflectionType $type, ReflectionClass $declaringClass): string
    {}

    /**
     * Get the string representation of the given type.
     *
     * @return list<array{typeHint:string,isPrimitive:bool}>
     */
    private static function getTypeInformation(ReflectionType $type, ReflectionClass $declaringClass): array
    {}
}
