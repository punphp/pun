<?php declare(strict_types=1);
/*
 * This file is part of phpunit/php-code-coverage.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace SebastianBergmann\CodeCoverage\Report;

use function date;
use function htmlspecialchars;
use function is_string;
use function round;
use DOMDocument;
use SebastianBergmann\CodeCoverage\Node\Directory;
use SebastianBergmann\CodeCoverage\Node\File;
use SebastianBergmann\CodeCoverage\Util\EnsuresUtf8;
use SebastianBergmann\CodeCoverage\Util\Filesystem;
use SebastianBergmann\CodeCoverage\Util\Xml;
use SebastianBergmann\CodeCoverage\WriteOperationFailedException;

/**
 * @internal This class is not covered by the backward compatibility promise for phpunit/php-code-coverage
 *
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for phpunit/php-code-coverage
 */
final readonly class Crap4j
{
    use EnsuresUtf8;
    private int $threshold;

    public function __construct(int $threshold = 30)
    {}

    /**
     * @param null|non-empty-string $target
     * @param null|non-empty-string $name
     *
     * @throws WriteOperationFailedException
     */
    public function process(Directory $report, ?string $target = null, ?string $name = null): string
    {}

    private function crapLoad(float $crapValue, int $cyclomaticComplexity, float $coveragePercent): float
    {}

    private function roundValue(float $value): float
    {}
}
