<?php declare(strict_types=1);
/*
 * This file is part of PHPUnit.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace PHPUnit\Framework\TestCase;

use const PHP_OUTPUT_HANDLER_CLEAN;
use const PHP_OUTPUT_HANDLER_FINAL;
use function array_any;
use function in_array;
use function ob_end_clean;
use function ob_get_contents;
use function ob_get_level;
use function ob_start;
use PHPUnit\Framework\Assert;
use PHPUnit\Framework\Exception;
use PHPUnit\Framework\ExpectationFailedException;

/**
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 *
 * @internal This class is not covered by the backward compatibility promise for PHPUnit
 */
final class OutputBuffer
{
    private string $output = '';

    /**
     * @var list<string>
     */
    private array $expectedRegularExpressions = [];

    /**
     * @var list<string>
     */
    private array $expectedStrings      = [];
    private bool $bufferingActive       = false;
    private int $bufferingLevel         = 0;
    private string $bufferingCaptured   = '';
    private bool $bufferingDestroyed    = false;
    private bool $retrievedForAssertion = false;

    public function expectRegularExpression(string $expectedRegularExpression): void
    {}

    public function expectString(string $expectedString): void
    {}

    public function conflictsWithExpectedString(string $expectedString): bool
    {}

    public function hasExpectation(): bool
    {}

    public function expectsOutput(): bool
    {}

    /**
     * @phpstan-assert-if-true non-empty-string $this->output()
     */
    public function hasUnexpectedOutput(): bool
    {}

    public function output(): string
    {}

    public function getActualOutputForAssertion(): string
    {}

    public function start(): void
    {}

    public function stop(): OutputBufferStopResult
    {}

    /**
     * @throws Exception
     * @throws ExpectationFailedException
     */
    public function performAssertions(): void
    {}
}
