<?php declare(strict_types=1);
/*
 * This file is part of phpunit/php-code-coverage.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace SebastianBergmann\CodeCoverage\Util;

use const DIRECTORY_SEPARATOR;
use function array_keys;
use function array_shift;
use function basename;
use function count;
use function dirname;
use function explode;
use function implode;
use function str_replace;
use function str_starts_with;
use function substr;
use SebastianBergmann\CodeCoverage\Data\ProcessedCodeCoverageData;

/**
 * @internal This class is not covered by the backward compatibility promise for phpunit/php-code-coverage
 *
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for phpunit/php-code-coverage
 */
final class PathReducer
{
    /**
     * Reduces the paths by cutting the longest common start path, renames the
     * files in the coverage data accordingly, and returns the common path.
     *
     * Returns an empty string when there are no covered files.
     */
    public function reduce(ProcessedCodeCoverageData $data): string
    {}
}
