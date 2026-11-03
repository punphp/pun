<?php declare(strict_types=1);
/*
 * This file is part of PHPUnit.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace PHPUnit\Runner\Baseline;

use function dirname;
use function file_put_contents;
use function is_dir;
use function realpath;
use function sprintf;
use XMLWriter;

/**
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 *
 * @internal This class is not covered by the backward compatibility promise for PHPUnit
 */
final readonly class Writer
{
    /**
     * @param non-empty-string $baselineFile
     *
     * @throws CannotWriteBaselineException
     */
    public function write(string $baselineFile, Baseline $baseline): void
    {}
}
