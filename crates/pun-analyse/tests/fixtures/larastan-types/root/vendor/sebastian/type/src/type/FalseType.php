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

/**
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for this library
 */
final class FalseType extends Type
{
    public function isAssignable(Type $other): bool
    {}

    /**
     * @return 'false'
     */
    public function name(): string
    {}

    public function allowsNull(): bool
    {}

    public function isFalse(): bool
    {}
}
