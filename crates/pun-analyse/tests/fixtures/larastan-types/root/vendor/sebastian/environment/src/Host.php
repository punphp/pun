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

use const PHP_OS_FAMILY;
use function assert;
use function function_exists;
use function getenv;
use function gethostname;
use function php_uname;
use function posix_geteuid;
use function posix_getpwuid;
use function trim;

final readonly class Host
{
    /**
     * @return non-empty-string
     */
    public function name(): string
    {}

    /**
     * @return non-empty-string
     */
    public function user(): string
    {}

    /**
     * @return non-empty-string
     */
    public function operatingSystem(): string
    {}
}
