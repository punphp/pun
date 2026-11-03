<?php declare(strict_types=1);
/*
 * This file is part of phpunit/php-code-coverage.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace SebastianBergmann\CodeCoverage\Driver;

use const XDEBUG_CC_BRANCH_CHECK;
use const XDEBUG_CC_DEAD_CODE;
use const XDEBUG_CC_UNUSED;
use const XDEBUG_FILTER_CODE_COVERAGE;
use const XDEBUG_PATH_INCLUDE;
use function extension_loaded;
use function in_array;
use function phpversion;
use function version_compare;
use function xdebug_get_code_coverage;
use function xdebug_info;
use function xdebug_set_filter;
use function xdebug_start_code_coverage;
use function xdebug_stop_code_coverage;
use SebastianBergmann\CodeCoverage\Data\RawCodeCoverageData;
use SebastianBergmann\CodeCoverage\Filter;

/**
 * @internal This class is not covered by the backward compatibility promise for phpunit/php-code-coverage
 *
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for phpunit/php-code-coverage
 *
 * @see https://xdebug.org/docs/code_coverage#xdebug_get_code_coverage
 *
 * @phpstan-import-type CodeCoverageWithoutPathCoverageType from RawCodeCoverageData as XdebugCodeCoverageWithoutPathCoverageType
 * @phpstan-import-type CodeCoverageWithPathCoverageType from RawCodeCoverageData as XdebugCodeCoverageWithPathCoverageType
 */
final class XdebugDriver extends Driver
{
    /**
     * @throws XdebugNotAvailableException
     * @throws XdebugNotEnabledException
     * @throws XdebugVersionNotSupportedException
     */
    public function __construct(Filter $filter)
    {}

    public function start(): void
    {}

    public function stop(): RawCodeCoverageData
    {}

    public function name(): string
    {}

    public function version(): string
    {}

    protected function canCollectBranchCoverage(): bool
    {}

    protected function canCollectPathCoverage(): bool
    {}

    /**
     * The shape of the data returned by xdebug_get_code_coverage() is
     * determined by the flags that were passed to xdebug_start_code_coverage()
     * in start(): when XDEBUG_CC_BRANCH_CHECK was set, branch and path coverage
     * are included.
     *
     * @param array<non-empty-string, mixed> $data
     *
     * @phpstan-assert XdebugCodeCoverageWithPathCoverageType $data
     */
    private function ensureWithPathCoverage(array $data): void
    {}

    /**
     * @param array<non-empty-string, mixed> $data
     *
     * @phpstan-assert XdebugCodeCoverageWithoutPathCoverageType $data
     *
     * @see ensureWithPathCoverage()
     */
    private function ensureWithoutPathCoverage(array $data): void
    {}

    /**
     * @throws XdebugNotAvailableException
     * @throws XdebugNotEnabledException
     * @throws XdebugVersionNotSupportedException
     */
    private function ensureXdebugIsAvailable(): void
    {}
}
