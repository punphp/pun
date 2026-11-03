<?php declare(strict_types=1);
/*
 * This file is part of phpunit/php-code-coverage.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace SebastianBergmann\CodeCoverage\Report\Xml;

use XMLWriter;

/**
 * @internal This class is not covered by the backward compatibility promise for phpunit/php-code-coverage
 *
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for phpunit/php-code-coverage
 */
final readonly class Coverage
{
    private XMLWriter $xmlWriter;
    private string $line;

    public function __construct(
        XMLWriter $xmlWriter,
        string $line
    ) {}

    /**
     * @param array<non-empty-string, positive-int> $tests map of test id to the number of times the test executed the line
     */
    public function finalize(array $tests): void
    {}
}
