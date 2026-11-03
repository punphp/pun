<?php declare(strict_types=1);
/*
 * This file is part of phpunit/php-invoker.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace SebastianBergmann\Invoker;

use const PHP_EOL;
use function extension_loaded;
use function function_exists;
use function implode;
use RuntimeException;

final class ProcessControlExtensionNotLoadedException extends RuntimeException implements Exception
{
    public function __construct()
    {}
}
