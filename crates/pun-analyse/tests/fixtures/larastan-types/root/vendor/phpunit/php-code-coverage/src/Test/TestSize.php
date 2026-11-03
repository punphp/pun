<?php declare(strict_types=1);
/*
 * This file is part of phpunit/php-code-coverage.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace SebastianBergmann\CodeCoverage\Test;

/**
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for phpunit/php-code-coverage
 */
enum TestSize: string
{
    public function isKnown(): bool
    {}

    public function isUnknown(): bool
    {}

    public function isSmall(): bool
    {}

    public function isMedium(): bool
    {}

    public function isLarge(): bool
    {}

    public function isGreaterThan(self $other): bool
    {}

    public function asString(): string
    {}

    private function order(): int
    {}

    case Unknown = 'unknown';
    case Small   = 'small';
    case Medium  = 'medium';
    case Large   = 'large';
}
