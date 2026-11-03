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

use function sprintf;

/**
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 *
 * @internal This class is not covered by the backward compatibility promise for PHPUnit
 */
final readonly class PlainTextRenderer
{
    /**
     * @param array<class-string, TestResultCollection> $tests
     */
    public function render(array $tests): string
    {}

    /**
     * @return array<string, ' '|'x'>
     */
    private function reduce(TestResultCollection $tests): array
    {}
}
