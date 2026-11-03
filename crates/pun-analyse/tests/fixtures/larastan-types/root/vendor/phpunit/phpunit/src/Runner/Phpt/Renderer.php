<?php declare(strict_types=1);
/*
 * This file is part of PHPUnit.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace PHPUnit\Runner\Phpt;

use function assert;
use function defined;
use function dirname;
use function file_put_contents;
use function str_replace;
use function var_export;
use SebastianBergmann\Template\InvalidArgumentException;
use SebastianBergmann\Template\Template;

/**
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 *
 * @internal This class is not covered by the backward compatibility promise for PHPUnit
 *
 * @see https://qa.php.net/phpt_details.php
 *
 * @phpstan-import-type CoverageFiles from TestCase
 */
final readonly class Renderer
{
    /**
     * @param non-empty-string $phptFile
     * @param non-empty-string $code
     *
     * @return non-empty-string
     */
    public function render(string $phptFile, string $code): string
    {}

    /**
     * @param non-empty-string $job
     * @param CoverageFiles    $files
     *
     * @param-out non-empty-string $job
     *
     * @throws InvalidArgumentException
     */
    public function renderForCoverage(string &$job, bool $branchCoverage, bool $pathCoverage, ?string $codeCoverageCacheDirectory, string $bootstrap, array $files): void
    {}
}
