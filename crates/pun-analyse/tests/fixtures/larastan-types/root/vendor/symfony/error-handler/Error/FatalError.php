<?php

/*
 * This file is part of the Symfony package.
 *
 * (c) Fabien Potencier <fabien@symfony.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace Symfony\Component\ErrorHandler\Error;

class FatalError extends \Error
{
    /**
     * @param array $error An array as returned by error_get_last()
     */
    public function __construct(
        string $message,
        int $code,
        private array $error,
        ?int $traceOffset = null,
        bool $traceArgs = true,
        ?array $trace = null,
    ) {}

    public function getError(): array
    {}
}
