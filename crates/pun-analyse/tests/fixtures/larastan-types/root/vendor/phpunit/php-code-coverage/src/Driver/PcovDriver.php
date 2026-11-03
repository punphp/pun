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

use const pcov\inclusive;
use function array_intersect;
use function extension_loaded;
use function pcov\clear;
use function pcov\collect;
use function pcov\start;
use function pcov\stop;
use function pcov\waiting;
use function phpversion;
use SebastianBergmann\CodeCoverage\Data\RawCodeCoverageData;
use SebastianBergmann\CodeCoverage\Filter;

/**
 * @internal This class is not covered by the backward compatibility promise for phpunit/php-code-coverage
 *
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for phpunit/php-code-coverage
 */
final class PcovDriver extends Driver
{
    private readonly Filter $filter;

    /**
     * @throws PcovNotAvailableException
     */
    public function __construct(Filter $filter)
    {}

    /**
     * @codeCoverageIgnore
     */
    public function start(): void
    {}

    public function stop(): RawCodeCoverageData
    {}

    public function name(): string
    {}

    public function version(): string
    {}

    /**
     * @throws PcovNotAvailableException
     */
    private function ensurePcovIsAvailable(): void
    {}
}
