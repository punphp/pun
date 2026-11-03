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

use Exception;
use PHPUnit\Framework\ExpectationFailedException;
use PHPUnit\Framework\MockObject\Invocation as BaseInvocation;

/**
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 *
 * @internal This class is not covered by the backward compatibility promise for PHPUnit
 */
final class IndexedParameters implements ParametersRule
{
    private Parameters $parameters;

    /**
     * @param array<int|string, mixed> $parameters
     *
     * @throws \PHPUnit\Framework\Exception
     */
    public function __construct(array $parameters, private int $index, private bool $strict)
    {}

    /**
     * @throws Exception
     */
    public function apply(BaseInvocation $invocation): void
    {}

    /**
     * Checks if the invocation $invocation matches the current rules on underlying parameters.
     *
     * @throws ExpectationFailedException
     */
    public function verify(): void
    {}

    public function useAssertionCount(bool $useAssertionCount): void
    {}

    public function at(): int
    {}

    public function isStrict(): bool
    {}
}
