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

use function array_any;
use function in_array;
use function preg_match;
use function sprintf;
use PHPUnit\Framework\ExpectationFailedException;
use PHPUnit\Framework\TestCase;
use PHPUnit\Runner\DeprecationCollector\Facade as DeprecationCollector;

/**
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 *
 * @internal This class is not covered by the backward compatibility promise for PHPUnit
 */
final class DeprecationExpectation
{
    /**
     * @var list<non-empty-string>
     */
    private array $expectedMessages = [];

    /**
     * @var list<non-empty-string>
     */
    private array $expectedMessageRegularExpressions = [];

    /**
     * @param non-empty-string $expectedMessage
     */
    public function expectMessage(string $expectedMessage): void
    {}

    /**
     * @param non-empty-string $expectedMessageRegularExpression
     */
    public function expectMessageMatches(string $expectedMessageRegularExpression): void
    {}

    /**
     * @throws ExpectationFailedException
     */
    public function verify(TestCase $test): void
    {}
}
