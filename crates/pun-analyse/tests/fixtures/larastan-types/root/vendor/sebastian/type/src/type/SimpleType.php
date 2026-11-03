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

use function strtolower;

/**
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for this library
 */
final class SimpleType extends Type
{
    /**
     * @var non-empty-string
     */
    private string $name;
    private bool $allowsNull;
    private mixed $value;

    /**
     * @param non-empty-string $name
     */
    public function __construct(string $name, bool $nullable, mixed $value = null)
    {}

    public function isAssignable(Type $other): bool
    {}

    /**
     * @return non-empty-string
     */
    public function name(): string
    {}

    public function allowsNull(): bool
    {}

    public function value(): mixed
    {}

    public function isSimple(): bool
    {}

    /**
     * @param non-empty-string $name
     *
     * @return non-empty-string
     */
    private function normalize(string $name): string
    {}
}
