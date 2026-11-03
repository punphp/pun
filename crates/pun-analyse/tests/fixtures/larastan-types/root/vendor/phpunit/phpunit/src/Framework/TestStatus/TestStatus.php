<?php declare(strict_types=1);
/*
 * This file is part of PHPUnit.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace PHPUnit\Framework\TestStatus;

/**
 * @immutable
 *
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 *
 * @internal This class is not covered by the backward compatibility promise for PHPUnit
 */
abstract readonly class TestStatus
{
    private string $message;

    public static function from(int $status): self
    {}

    public static function unknown(): self
    {}

    public static function success(): self
    {}

    public static function skipped(string $message = ''): self
    {}

    public static function incomplete(string $message = ''): self
    {}

    public static function notice(string $message = ''): self
    {}

    public static function deprecation(string $message = ''): self
    {}

    public static function failure(string $message = ''): self
    {}

    public static function error(string $message = ''): self
    {}

    public static function warning(string $message = ''): self
    {}

    public static function risky(string $message = ''): self
    {}

    private function __construct(string $message = '')
    {}

    /**
     * @phpstan-assert-if-true Known $this
     */
    public function isKnown(): bool
    {}

    /**
     * @phpstan-assert-if-true Unknown $this
     */
    public function isUnknown(): bool
    {}

    /**
     * @phpstan-assert-if-true Success $this
     */
    public function isSuccess(): bool
    {}

    /**
     * @phpstan-assert-if-true Skipped $this
     */
    public function isSkipped(): bool
    {}

    /**
     * @phpstan-assert-if-true Incomplete $this
     */
    public function isIncomplete(): bool
    {}

    /**
     * @phpstan-assert-if-true Notice $this
     */
    public function isNotice(): bool
    {}

    /**
     * @phpstan-assert-if-true Deprecation $this
     */
    public function isDeprecation(): bool
    {}

    /**
     * @phpstan-assert-if-true Failure $this
     */
    public function isFailure(): bool
    {}

    /**
     * @phpstan-assert-if-true Error $this
     */
    public function isError(): bool
    {}

    /**
     * @phpstan-assert-if-true Warning $this
     */
    public function isWarning(): bool
    {}

    /**
     * @phpstan-assert-if-true Risky $this
     */
    public function isRisky(): bool
    {}

    public function message(): string
    {}

    public function isMoreImportantThan(self $other): bool
    {}

    abstract public function asInt(): int;

    abstract public function asString(): string;
}
