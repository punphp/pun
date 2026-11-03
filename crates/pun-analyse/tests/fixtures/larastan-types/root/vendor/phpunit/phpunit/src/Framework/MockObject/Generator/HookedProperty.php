<?php declare(strict_types=1);
/*
 * This file is part of PHPUnit.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace PHPUnit\Framework\MockObject\Generator;

use SebastianBergmann\Type\Type;

/**
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 *
 * @internal This class is not covered by the backward compatibility promise for PHPUnit
 */
final readonly class HookedProperty
{
    /**
     * @var non-empty-string
     */
    private string $name;
    private Type $type;
    private bool $getHook;
    private bool $setHook;
    private bool $finalGetHook;
    private bool $finalSetHook;
    private ?Type $setterType;

    /**
     * @param non-empty-string $name
     */
    public function __construct(string $name, Type $type, bool $getHook, bool $setHook, bool $finalGetHook, bool $finalSetHook, ?Type $setterType)
    {}

    public function name(): string
    {}

    public function type(): Type
    {}

    public function hasGetHook(): bool
    {}

    public function hasSetHook(): bool
    {}

    public function shouldGenerateGetHook(): bool
    {}

    public function shouldGenerateSetHook(): bool
    {}

    public function hasSetterType(): bool
    {}

    /**
     * @throws RuntimeException
     */
    public function setterType(): Type
    {}
}
