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

use const PHP_EOL;
use function getmypid;
use function register_shutdown_function;
use function rtrim;

/**
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 *
 * @internal This class is not covered by the backward compatibility promise for PHPUnit
 */
final class ShutdownHandler
{
    private static bool $registered = false;
    private static string $message  = '';

    public static function setMessage(string $message): void
    {}

    public static function resetMessage(): void
    {}

    private static function register(): void
    {}
}
