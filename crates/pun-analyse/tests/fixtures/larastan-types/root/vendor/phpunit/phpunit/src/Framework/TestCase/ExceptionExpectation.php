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

use function class_exists;
use function is_string;
use function sprintf;
use PHPUnit\Framework\Assert;
use PHPUnit\Framework\AssertionFailedError;
use PHPUnit\Framework\Constraint\Constraint;
use PHPUnit\Framework\Constraint\Exception as ExceptionConstraint;
use PHPUnit\Framework\Constraint\ExceptionCode;
use PHPUnit\Framework\Constraint\ExceptionMessageIs;
use PHPUnit\Framework\Constraint\ExceptionMessageIsOrContains;
use PHPUnit\Framework\Constraint\ExceptionMessageMatchesRegularExpression;
use PHPUnit\Framework\Exception;
use PHPUnit\Framework\ExpectationFailedException;
use PHPUnit\Framework\TestCase;
use ReflectionClass;
use Throwable;

/**
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 *
 * @internal This class is not covered by the backward compatibility promise for PHPUnit
 */
final class ExceptionExpectation
{
    private ?string $expectedException                = null;
    private ?string $expectedMessage                  = null;
    private ?Constraint $expectedMessageConstraint    = null;
    private ?string $expectedMessageRegularExpression = null;
    private null|int|string $expectedCode             = null;

    /**
     * @param class-string<Throwable> $exception
     */
    public function expectClass(string $exception): void
    {}

    public function expectCode(int|string $code): void
    {}

    public function expectMessageIs(string $message): void
    {}

    public function expectMessageIsOrContains(string $message): void
    {}

    public function expectMessageMatches(string $regularExpression): void
    {}

    /**
     * @throws Exception
     */
    public function shouldBeVerifiedFor(Throwable $throwable): bool
    {}

    /**
     * @throws ExpectationFailedException
     */
    public function verify(Throwable $exception): void
    {}

    /**
     * @throws AssertionFailedError
     */
    public function assertWasRaised(TestCase $test): void
    {}
}
