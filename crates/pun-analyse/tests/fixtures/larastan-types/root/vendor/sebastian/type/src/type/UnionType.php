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

use function array_any;
use function assert;
use function count;
use function implode;
use function sort;

/**
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for this library
 */
final class UnionType extends Type
{
    /**
     * @var non-empty-list<Type>
     */
    private array $types;

    /**
     * @throws RuntimeException
     */
    public function __construct(Type ...$types)
    {}

    public function isAssignable(Type $other): bool
    {}

    /**
     * @return non-empty-string
     */
    public function asString(): string
    {}

    /**
     * @return non-empty-string
     */
    public function name(): string
    {}

    public function allowsNull(): bool
    {}

    public function isUnion(): bool
    {}

    public function containsIntersectionTypes(): bool
    {}

    /**
     * @return non-empty-list<Type>
     */
    public function types(): array
    {}

    /**
     * @throws RuntimeException
     */
    private function ensureMinimumOfTwoTypes(Type ...$types): void
    {}

    /**
     * @throws RuntimeException
     */
    private function ensureOnlyValidTypes(Type ...$types): void
    {}
}
