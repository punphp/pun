<?php

/*
 * This file is part of the Symfony package.
 *
 * (c) Fabien Potencier <fabien@symfony.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace Symfony\Component\HttpKernel\Log;

use Monolog\Logger;

/**
 * @author Nicolas Grekas <p@tchwork.com>
 */
class DebugLoggerConfigurator
{
    private ?object $processor = null;

    public function __construct(callable $processor, ?bool $enable = null)
    {}

    public function pushDebugLogger(Logger $logger): void
    {}

    public static function getDebugLogger(mixed $logger): ?DebugLoggerInterface
    {}
}
