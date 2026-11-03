<?php

/*
 * This file is part of the Symfony package.
 *
 * (c) Fabien Potencier <fabien@symfony.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace Symfony\Component\HttpKernel\Exception;

/**
 * HttpException.
 *
 * @author Kris Wallsmith <kris@symfony.com>
 */
class HttpException extends \RuntimeException implements HttpExceptionInterface
{
    public function __construct(
        private int $statusCode,
        string $message = '',
        ?\Throwable $previous = null,
        private array $headers = [],
        int $code = 0,
    ) {}

    public static function fromStatusCode(int $statusCode, string $message = '', ?\Throwable $previous = null, array $headers = [], int $code = 0): self
    {}

    public function getStatusCode(): int
    {}

    public function getHeaders(): array
    {}

    public function setHeaders(array $headers): void
    {}
}
