<?php declare(strict_types=1);
/*
 * This file is part of sebastian/environment.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace SebastianBergmann\Environment;

use const DIRECTORY_SEPARATOR;
use const STDIN;
use const STDOUT;
use function assert;
use function defined;
use function fclose;
use function fstat;
use function function_exists;
use function getenv;
use function in_array;
use function is_int;
use function is_resource;
use function is_string;
use function posix_isatty;
use function preg_match;
use function proc_close;
use function proc_open;
use function sapi_windows_vt100_support;
use function shell_exec;
use function stream_get_contents;
use function stream_isatty;
use function strtoupper;
use function trim;

final class Console
{
    /**
     * @var int
     */
    public const int STDIN = 0;

    /**
     * @var int
     */
    public const int STDOUT = 1;

    /**
     * @var int
     */
    public const int STDERR = 2;

    /**
     * Returns true if STDOUT supports colorization.
     *
     * This code has been copied and adapted from
     * Symfony\Component\Console\Output\StreamOutput.
     */
    public function hasColorSupport(): bool
    {}

    /**
     * Returns the number of columns of the terminal.
     *
     * @codeCoverageIgnore
     */
    public function getNumberOfColumns(): int
    {}

    /**
     * Returns if the file descriptor is an interactive terminal or not.
     *
     * Normally, we want to use a resource as a parameter, yet sadly it's not always available,
     * eg when running code in interactive console (`php -a`), STDIN/STDOUT/STDERR constants are not defined.
     *
     * @param int|resource $fileDescriptor
     */
    public function isInteractive(mixed $fileDescriptor = self::STDOUT): bool
    {}

    private function isWindows(): bool
    {}

    /**
     * @codeCoverageIgnore
     */
    private function getNumberOfColumnsInteractive(): int
    {}

    /**
     * @codeCoverageIgnore
     */
    private function getNumberOfColumnsWindows(): int
    {}
}
