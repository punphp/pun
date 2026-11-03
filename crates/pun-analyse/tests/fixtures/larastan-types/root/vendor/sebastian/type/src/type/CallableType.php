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

use function class_exists;
use function count;
use function explode;
use function function_exists;
use function is_array;
use function is_object;
use function is_string;
use function method_exists;
use function str_contains;
use Closure;
use ReflectionMethod;

/**
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for this library
 */
final class CallableType extends Type
{
    private bool $allowsNull;

    public function __construct(bool $nullable)
    {}

    public function isAssignable(Type $other): bool
    {}

    /**
     * @return 'callable'
     */
    public function name(): string
    {}

    public function allowsNull(): bool
    {}

    public function isCallable(): bool
    {}

    private function isClosure(ObjectType $type): bool
    {}

    private function hasInvokeMethod(ObjectType $type): bool
    {}

    private function isFunction(SimpleType $type): bool
    {}

    private function isObjectCallback(SimpleType $type): bool
    {}

    private function isClassCallback(SimpleType $type): bool
    {}
}
