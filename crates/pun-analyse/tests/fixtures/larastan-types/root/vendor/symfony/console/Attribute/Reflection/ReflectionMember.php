<?php

/*
 * This file is part of the Symfony package.
 *
 * (c) Fabien Potencier <fabien@symfony.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace Symfony\Component\Console\Attribute\Reflection;

use Symfony\Component\String\UnicodeString;

/**
 * @internal
 */
class ReflectionMember
{
    public function __construct(
        private readonly \ReflectionParameter|\ReflectionProperty $member,
    ) {}

    /**
     * @template T of object
     *
     * @param class-string<T> $class
     *
     * @return T|null
     */
    public function getAttribute(string $class): ?object
    {}

    /**
     * @template T of object
     *
     * @param class-string<T> $class
     *
     * @return list<T>
     */
    public function getAttributes(string $class): array
    {}

    public function getSourceName(): string
    {}

    public function getSourceThis(): ?object
    {}

    public function getType(): ?\ReflectionType
    {}

    public function getName(): string
    {}

    public function hasDefaultValue(): bool
    {}

    public function getDefaultValue(): mixed
    {}

    public function isNullable(): bool
    {}

    public function getMemberName(): string
    {}

    public function isParameter(): bool
    {}

    public function isVariadic(): bool
    {}

    public function isProperty(): bool
    {}

    public function getMember(): \ReflectionParameter|\ReflectionProperty
    {}

    public function getInputName(): string
    {}
}
