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

use function is_subclass_of;
use function strcasecmp;

/**
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for this library
 */
final class StaticType extends Type
{
    private TypeName $className;
    private bool $allowsNull;

    public function __construct(TypeName $className, bool $allowsNull)
    {}

    public function isAssignable(Type $other): bool
    {}

    /**
     * @return 'static'
     */
    public function name(): string
    {}

    public function allowsNull(): bool
    {}

    public function isStatic(): bool
    {}
}
