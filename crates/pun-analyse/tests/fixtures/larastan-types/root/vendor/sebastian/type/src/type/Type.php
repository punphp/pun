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

use function gettype;
use function is_object;
use function strtolower;

/**
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for this library
 */
abstract class Type
{
    public static function fromValue(mixed $value, bool $allowsNull): self
    {}

    /**
     * @param non-empty-string $typeName
     */
    public static function fromName(string $typeName, bool $allowsNull): self
    {}

    public function asString(): string
    {}

    /**
     * @phpstan-assert-if-true CallableType $this
     */
    public function isCallable(): bool
    {}

    /**
     * @phpstan-assert-if-true TrueType $this
     */
    public function isTrue(): bool
    {}

    /**
     * @phpstan-assert-if-true FalseType $this
     */
    public function isFalse(): bool
    {}

    /**
     * @phpstan-assert-if-true GenericObjectType $this
     */
    public function isGenericObject(): bool
    {}

    /**
     * @phpstan-assert-if-true IntersectionType $this
     */
    public function isIntersection(): bool
    {}

    /**
     * @phpstan-assert-if-true IterableType $this
     */
    public function isIterable(): bool
    {}

    /**
     * @phpstan-assert-if-true MixedType $this
     */
    public function isMixed(): bool
    {}

    /**
     * @phpstan-assert-if-true NeverType $this
     */
    public function isNever(): bool
    {}

    /**
     * @phpstan-assert-if-true NullType $this
     */
    public function isNull(): bool
    {}

    /**
     * @phpstan-assert-if-true ObjectType $this
     */
    public function isObject(): bool
    {}

    /**
     * @phpstan-assert-if-true SimpleType $this
     */
    public function isSimple(): bool
    {}

    /**
     * @phpstan-assert-if-true StaticType $this
     */
    public function isStatic(): bool
    {}

    /**
     * @phpstan-assert-if-true UnionType $this
     */
    public function isUnion(): bool
    {}

    /**
     * @phpstan-assert-if-true UnknownType $this
     */
    public function isUnknown(): bool
    {}

    /**
     * @phpstan-assert-if-true VoidType $this
     */
    public function isVoid(): bool
    {}

    abstract public function isAssignable(self $other): bool;

    /**
     * @return non-empty-string
     */
    abstract public function name(): string;

    abstract public function allowsNull(): bool;
}
