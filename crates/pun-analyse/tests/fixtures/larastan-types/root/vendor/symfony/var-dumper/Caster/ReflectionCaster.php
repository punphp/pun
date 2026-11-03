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
 * Casts Reflector related classes to array representation.
 *
 * @author Nicolas Grekas <p@tchwork.com>
 *
 * @final
 *
 * @internal
 */
class ReflectionCaster
{
    public const UNSET_CLOSURE_FILE_INFO = ['Closure' => __CLASS__.'::unsetClosureFileInfo'];

    private const EXTRA_MAP = [
        'docComment' => 'getDocComment',
        'extension' => 'getExtensionName',
        'isDisabled' => 'isDisabled',
        'isDeprecated' => 'isDeprecated',
        'isInternal' => 'isInternal',
        'isUserDefined' => 'isUserDefined',
        'isGenerator' => 'isGenerator',
        'isVariadic' => 'isVariadic',
    ];

    public static function castClosure(\Closure $c, array $a, Stub $stub, bool $isNested, int $filter = 0): array
    {}

    public static function unsetClosureFileInfo(\Closure $c, array $a): array
    {}

    public static function castGenerator(\Generator $c, array $a, Stub $stub, bool $isNested): array
    {}

    public static function castType(\ReflectionType $c, array $a, Stub $stub, bool $isNested): array
    {}

    public static function castAttribute(\ReflectionAttribute $c, array $a, Stub $stub, bool $isNested): array
    {}

    public static function castReflectionGenerator(\ReflectionGenerator $c, array $a, Stub $stub, bool $isNested): array
    {}

    public static function castClass(\ReflectionClass $c, array $a, Stub $stub, bool $isNested, int $filter = 0): array
    {}

    public static function castFunctionAbstract(\ReflectionFunctionAbstract $c, array $a, Stub $stub, bool $isNested, int $filter = 0): array
    {}

    public static function castClassConstant(\ReflectionClassConstant $c, array $a, Stub $stub, bool $isNested): array
    {}

    public static function castMethod(\ReflectionMethod $c, array $a, Stub $stub, bool $isNested): array
    {}

    public static function castParameter(\ReflectionParameter $c, array $a, Stub $stub, bool $isNested): array
    {}

    public static function castProperty(\ReflectionProperty $c, array $a, Stub $stub, bool $isNested): array
    {}

    public static function castReference(\ReflectionReference $c, array $a, Stub $stub, bool $isNested): array
    {}

    public static function castExtension(\ReflectionExtension $c, array $a, Stub $stub, bool $isNested): array
    {}

    public static function castZendExtension(\ReflectionZendExtension $c, array $a, Stub $stub, bool $isNested): array
    {}

    public static function getSignature(array $a): string
    {}

    private static function addExtra(array &$a, \Reflector $c): void
    {}

    private static function addMap(array &$a, object $c, array $map, string $prefix = Caster::PREFIX_VIRTUAL): void
    {}

    private static function addAttributes(array &$a, \Reflector $c, string $prefix = Caster::PREFIX_VIRTUAL): void
    {}
}
