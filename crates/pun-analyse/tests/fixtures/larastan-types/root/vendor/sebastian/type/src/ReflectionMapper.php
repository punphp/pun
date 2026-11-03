<?php declare(strict_types=1);
/*
 * This file is part of sebastian/type.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace SebastianBergmann\Type;

use function array_filter;
use function assert;
use ReflectionFunction;
use ReflectionIntersectionType;
use ReflectionMethod;
use ReflectionNamedType;
use ReflectionProperty;
use ReflectionType;
use ReflectionUnionType;

/**
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for this library
 */
final class ReflectionMapper
{
    /**
     * @return list<Parameter>
     */
    public function fromParameterTypes(ReflectionFunction|ReflectionMethod $reflector): array
    {}

    public function fromReturnType(ReflectionFunction|ReflectionMethod $reflector): Type
    {}

    public function fromPropertyType(ReflectionProperty $reflector): Type
    {}

    private function mapType(ReflectionType $type, ReflectionFunction|ReflectionMethod|ReflectionProperty $reflector): Type
    {}

    private function mapNamedType(ReflectionNamedType $type, ReflectionFunction|ReflectionMethod|ReflectionProperty $reflector): Type
    {}

    private function mapUnionType(ReflectionUnionType $type, ReflectionFunction|ReflectionMethod|ReflectionProperty $reflector): Type
    {}

    private function mapIntersectionType(ReflectionIntersectionType $type, ReflectionFunction|ReflectionMethod|ReflectionProperty $reflector): Type
    {}
}
