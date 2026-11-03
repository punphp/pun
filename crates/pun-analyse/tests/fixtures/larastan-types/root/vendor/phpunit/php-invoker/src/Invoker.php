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

use const SIGALRM;
use function call_user_func_array;
use function extension_loaded;
use function function_exists;
use function pcntl_alarm;
use function pcntl_async_signals;
use function pcntl_signal;
use function sprintf;
use Throwable;

final class Invoker
{
    /**
     * @param array<mixed> $arguments
     *
     * @throws Throwable
     */
    public function invoke(callable $callable, array $arguments, int $timeout): mixed
    {}

    public function canInvokeWithTimeout(): bool
    {}
}
