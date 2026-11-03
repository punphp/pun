<?php

/*
 * This file is part of the Symfony package.
 *
 * (c) Fabien Potencier <fabien@symfony.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace Symfony\Component\VarDumper\Caster;

use Symfony\Component\VarDumper\Cloner\Stub;

/**
 * Helper for filtering out properties in casters.
 *
 * @author Nicolas Grekas <p@tchwork.com>
 *
 * @final
 */
class Caster
{
    public const EXCLUDE_VERBOSE = 1;
    public const EXCLUDE_VIRTUAL = 2;
    public const EXCLUDE_DYNAMIC = 4;
    public const EXCLUDE_PUBLIC = 8;
    public const EXCLUDE_PROTECTED = 16;
    public const EXCLUDE_PRIVATE = 32;
    public const EXCLUDE_NULL = 64;
    public const EXCLUDE_EMPTY = 128;
    public const EXCLUDE_NOT_IMPORTANT = 256;
    public const EXCLUDE_STRICT = 512;
    public const EXCLUDE_UNINITIALIZED = 1024;

    public const PREFIX_VIRTUAL = "\0~\0";
    public const PREFIX_DYNAMIC = "\0+\0";
    public const PREFIX_PROTECTED = "\0*\0";
    // usage: sprintf(Caster::PATTERN_PRIVATE, $class, $property)
    public const PATTERN_PRIVATE = "\0%s\0%s";

    private static array $classProperties = [];

    /**
     * Casts objects to arrays and adds the dynamic property prefix.
     *
     * @param bool $hasDebugInfo Whether the __debugInfo method exists on $obj or not
     *
     * @internal
     */
    public static function castObject(object $obj, string $class, bool $hasDebugInfo = false, ?string $debugClass = null): array
    {}

    /**
     * Filters out the specified properties.
     *
     * By default, a single match in the $filter bit field filters properties out, following an "or" logic.
     * When EXCLUDE_STRICT is set, an "and" logic is applied: all bits must match for a property to be removed.
     *
     * @param array    $a                The array containing the properties to filter
     * @param int      $filter           A bit field of Caster::EXCLUDE_* constants specifying which properties to filter out
     * @param string[] $listedProperties List of properties to exclude when Caster::EXCLUDE_VERBOSE is set, and to preserve when Caster::EXCLUDE_NOT_IMPORTANT is set
     * @param int|null &$count           Set to the number of removed properties
     */
    public static function filter(array $a, int $filter, array $listedProperties = [], ?int &$count = 0): array
    {}

    /**
     * @internal
     */
    public static function castPhpIncompleteClass(\__PHP_Incomplete_Class $c, array $a, Stub $stub, bool $isNested): array
    {}

    private static function getClassProperties(\ReflectionClass $class): array
    {}
}
