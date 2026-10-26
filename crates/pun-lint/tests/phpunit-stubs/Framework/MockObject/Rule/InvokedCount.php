<?php declare(strict_types=1);
/*
 * This file is part of PHPUnit.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace PHPUnit\Framework\MockObject\Rule;

use function sprintf;
use PHPUnit\Framework\ExpectationFailedException;
use PHPUnit\Framework\MockObject\Invocation as BaseInvocation;

/**
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 *
 * @internal This class is not covered by the backward compatibility promise for PHPUnit
 */
final class InvokedCount extends InvocationOrder
{
    private readonly int $expectedCount;

    public function __construct(int $expectedCount)
    {
    }

    public function isNever(): bool
    {
    }

    public function toString(): string
    {
    }

    public function matches(BaseInvocation $invocation): bool
    {
    }

    /**
     * Verifies that the current expectation is valid. If everything is OK the
     * code should just return, if not it must throw an exception.
     *
     * @throws ExpectationFailedException
     */
    public function verify(): void
    {
    }

    /**
     * @throws ExpectationFailedException
     */
    protected function invokedDo(BaseInvocation $invocation): void
    {
    }
}
