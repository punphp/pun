<?php declare(strict_types=1);
/*
 * This file is part of PHPUnit.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace PHPUnit\Framework\MockObject;

/**
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 *
 * @internal This class is not covered by the backward compatibility promise for PHPUnit
 */
final class TestDoubleState
{
    /**
     * @var list<ConfigurableMethod>
     */
    private readonly array $configurableMethods;

    /**
     * @var class-string
     */
    private readonly string $className;

    /**
     * @var ?non-empty-string
     */
    private readonly ?string $displayName;
    private readonly bool $generateReturnValues;
    private readonly bool $isMockObject;
    private ?InvocationHandler $invocationHandler = null;

    /**
     * @param list<ConfigurableMethod> $configurableMethods
     * @param class-string             $className
     * @param ?non-empty-string        $displayName
     */
    public function __construct(array $configurableMethods, string $className, bool $generateReturnValues, bool $isMockObject = false, ?string $displayName = null)
    {}

    public function invocationHandler(): InvocationHandler
    {}

    public function cloneInvocationHandler(): void
    {}

    public function unsetInvocationHandler(): void
    {}

    /**
     * @return list<ConfigurableMethod>
     */
    public function configurableMethods(): array
    {}

    public function generateReturnValues(): bool
    {}
}
