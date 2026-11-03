<?php declare(strict_types=1);
/*
 * This file is part of phpunit/php-code-coverage.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace SebastianBergmann\CodeCoverage;

use function rtrim;
use RuntimeException;

/**
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for phpunit/php-code-coverage
 */
final class UnintentionallyCoveredCodeException extends RuntimeException implements Exception
{
    /**
     * @var list<string>
     */
    private readonly array $unintentionallyCoveredUnits;

    /**
     * @param list<string> $unintentionallyCoveredUnits
     */
    public function __construct(array $unintentionallyCoveredUnits)
    {}

    /**
     * @return list<string>
     */
    public function getUnintentionallyCoveredUnits(): array
    {}

    private function toString(): string
    {}
}
