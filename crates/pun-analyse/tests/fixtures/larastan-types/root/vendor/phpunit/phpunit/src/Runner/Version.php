<?php declare(strict_types=1);
/*
 * This file is part of PHPUnit.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace PHPUnit\Runner;

use function array_slice;
use function assert;
use function dirname;
use function explode;
use function implode;
use function str_contains;
use SebastianBergmann\Version as VersionId;

/**
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 */
final class Version
{
    private static string $pharVersion = '';
    private static string $version     = '';

    /**
     * @return non-empty-string
     */
    public static function id(): string
    {}

    /**
     * @return non-empty-string
     */
    public static function series(): string
    {}

    /**
     * @return positive-int
     */
    public static function majorVersionNumber(): int
    {}

    /**
     * @return non-empty-string
     */
    public static function getVersionString(): string
    {}
}
