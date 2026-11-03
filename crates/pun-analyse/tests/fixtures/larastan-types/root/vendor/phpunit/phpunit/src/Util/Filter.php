<?php declare(strict_types=1);
/*
 * This file is part of PHPUnit.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace PHPUnit\Util;

use function array_any;
use function array_unshift;
use function defined;
use function in_array;
use function is_array;
use function is_file;
use function is_string;
use function realpath;
use function sprintf;
use function str_starts_with;
use PHPUnit\Framework\Exception;
use PHPUnit\Framework\PhptAssertionFailedError;
use Throwable;

/**
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 *
 * @internal This class is not covered by the backward compatibility promise for PHPUnit
 */
final readonly class Filter
{
    /**
     * @throws Exception
     */
    public static function stackTraceFromThrowableAsString(Throwable $t, bool $unwrap = true): string
    {}

    /**
     * @param list<array{file?: string, line?: int, ...}> $frames
     */
    private static function stackTraceAsString(array $frames): string
    {}

    /**
     * @param array{file?: string, ...<mixed>} $frame
     */
    private static function shouldPrintFrame(array $frame, false|string $prefix, ExcludeList $excludeList): bool
    {}

    private static function fileIsExcluded(string $file, ExcludeList $excludeList): bool
    {}

    /**
     * @param list<array{file?: string, line?: int, ...}> $trace
     */
    private static function frameExists(array $trace, string $file, int $line): bool
    {}
}
