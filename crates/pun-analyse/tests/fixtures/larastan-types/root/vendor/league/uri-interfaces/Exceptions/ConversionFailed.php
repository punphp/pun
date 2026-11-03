<?php

/**
 * League.Uri (https://uri.thephpleague.com)
 *
 * (c) Ignace Nyamagana Butera <nyamsprod@gmail.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

declare(strict_types=1);

namespace League\Uri\Exceptions;

use BackedEnum;
use League\Uri\Idna\Error;
use League\Uri\Idna\Result;
use Stringable;

final class ConversionFailed extends SyntaxError
{
    private function __construct(
        string $message,
        private readonly string $host,
        private readonly Result $result
    ) {}

    public static function dueToIdnError(BackedEnum|Stringable|string $host, Result $result): self
    {}

    public function getHost(): string
    {}

    public function getResult(): Result
    {}
}
