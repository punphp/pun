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

use const DIRECTORY_SEPARATOR;
use function basename;
use function dirname;
use function fclose;
use function fileowner;
use function fileperms;
use function fstat;
use function is_dir;
use function is_link;
use function mkdir;
use function realpath;
use function str_starts_with;
use function tmpfile;

/**
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 *
 * @internal This class is not covered by the backward compatibility promise for PHPUnit
 */
final readonly class Filesystem
{
    public static function createDirectory(string $directory): bool
    {}

    /**
     * Creates a directory that only the user with the given ID can access.
     *
     * A directory that already exists is only accepted when it is not a
     * symbolic link, is owned by the user with the given ID, and cannot be
     * written to by other users. Otherwise, another user could have created
     * it to control what is read from it.
     */
    public static function createPrivateDirectory(string $directory, int $userId): bool
    {}

    /**
     * Returns the ID of the user that owns the files that the current process
     * creates in the operating system's path used for temporary files.
     *
     * This is determined by creating a temporary file instead of by calling
     * posix_geteuid() so that it does not require the POSIX extension.
     */
    public static function currentUserId(): false|int
    {}

    /**
     * @param non-empty-string $path
     *
     * @return false|non-empty-string
     */
    public static function resolveStreamOrFile(string $path): false|string
    {}
}
