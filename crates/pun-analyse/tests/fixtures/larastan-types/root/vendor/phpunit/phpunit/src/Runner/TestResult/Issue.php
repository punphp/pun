<?php declare(strict_types=1);
/*
 * This file is part of PHPUnit.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace PHPUnit\TestRunner\TestResult\Issues;

use function array_values;
use function count;
use PHPUnit\Event\Code\Test;

/**
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 *
 * @internal This class is not covered by the backward compatibility promise for PHPUnit
 */
final class Issue
{
    /**
     * @var non-empty-string
     */
    private readonly string $file;

    /**
     * @var positive-int
     */
    private readonly int $line;

    /**
     * @var non-empty-string
     */
    private readonly string $description;

    /**
     * @var non-empty-array<non-empty-string, array{test: Test, count: int}>
     */
    private array $triggeringTests;

    /**
     * @var ?non-empty-string
     */
    private ?string $stackTrace;

    /**
     * @param non-empty-string  $file
     * @param positive-int      $line
     * @param non-empty-string  $description
     * @param ?non-empty-string $stackTrace
     */
    public static function from(string $file, int $line, string $description, Test $triggeringTest, ?string $stackTrace = null): self
    {}

    /**
     * @param non-empty-string  $file
     * @param positive-int      $line
     * @param non-empty-string  $description
     * @param ?non-empty-string $stackTrace
     */
    private function __construct(string $file, int $line, string $description, Test $triggeringTest, ?string $stackTrace)
    {}

    public function triggeredBy(Test $test): void
    {}

    /**
     * @return non-empty-string
     */
    public function file(): string
    {}

    /**
     * @return positive-int
     */
    public function line(): int
    {}

    /**
     * @return non-empty-string
     */
    public function description(): string
    {}

    /**
     * @return non-empty-array<non-empty-string, array{test: Test, count: int}>
     */
    public function triggeringTests(): array
    {}

    /**
     * @phpstan-assert-if-true !null $this->stackTrace
     */
    public function hasStackTrace(): bool
    {}

    /**
     * @return ?non-empty-string
     */
    public function stackTrace(): ?string
    {}

    public function triggeredInTest(): bool
    {}
}
