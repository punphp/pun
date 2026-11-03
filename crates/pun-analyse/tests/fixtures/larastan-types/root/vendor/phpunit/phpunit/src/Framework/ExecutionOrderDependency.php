<?php declare(strict_types=1);
/*
 * This file is part of PHPUnit.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace PHPUnit\Framework;

use function array_filter;
use function array_map;
use function array_values;
use function assert;
use function class_exists;
use function count;
use function explode;
use function in_array;
use function str_contains;
use PHPUnit\Metadata\DependsOnClass;
use PHPUnit\Metadata\DependsOnMethod;
use PHPUnit\Util\Test as TestUtil;
use ReflectionClass;
use Stringable;

/**
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 *
 * @internal This class is not covered by the backward compatibility promise for PHPUnit
 */
final class ExecutionOrderDependency implements Stringable
{
    private string $className  = '';
    private string $methodName = '';
    private readonly bool $shallowClone;
    private readonly bool $deepClone;

    public static function invalid(): self
    {}

    public static function forClass(DependsOnClass $metadata): self
    {}

    public static function forMethod(DependsOnMethod $metadata): self
    {}

    /**
     * @param list<ExecutionOrderDependency> $dependencies
     *
     * @return list<ExecutionOrderDependency>
     */
    public static function filterInvalid(array $dependencies): array
    {}

    /**
     * @param list<ExecutionOrderDependency> $existing
     * @param list<ExecutionOrderDependency> $additional
     *
     * @return list<ExecutionOrderDependency>
     */
    public static function mergeUnique(array $existing, array $additional): array
    {}

    /**
     * @param list<ExecutionOrderDependency> $left
     * @param list<ExecutionOrderDependency> $right
     *
     * @return list<ExecutionOrderDependency>
     */
    public static function diff(array $left, array $right): array
    {}

    public function __construct(string $classOrCallableName, ?string $methodName = null, bool $deepClone = false, bool $shallowClone = false)
    {}

    public function __toString(): string
    {}

    /**
     * @phpstan-assert-if-true non-empty-string $this->getTarget()
     */
    public function isValid(): bool
    {}

    public function shallowClone(): bool
    {}

    public function deepClone(): bool
    {}

    public function targetIsClass(): bool
    {}

    public function getTarget(): string
    {}

    public function getTargetClassName(): string
    {}

    public function targetIsCallableTestMethod(): bool
    {}
}
