<?php declare(strict_types=1);
/*
 * This file is part of PHPUnit.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace PHPUnit\Logging\TestDox;

use PHPUnit\Event\Code\TestMethod;
use PHPUnit\Event\Code\Throwable;
use PHPUnit\Framework\TestStatus\TestStatus;

/**
 * @immutable
 *
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 *
 * @internal This class is not covered by the backward compatibility promise for PHPUnit
 */
final readonly class TestResult
{
    private TestMethod $test;
    private TestStatus $status;
    private ?Throwable $throwable;

    public function __construct(TestMethod $test, TestStatus $status, ?Throwable $throwable)
    {}

    public function test(): TestMethod
    {}

    public function status(): TestStatus
    {}

    /**
     * @phpstan-assert-if-true !null $this->throwable
     */
    public function hasThrowable(): bool
    {}

    public function throwable(): ?Throwable
    {}
}
