<?php

/*
 * This file is part of the Symfony package.
 *
 * (c) Fabien Potencier <fabien@symfony.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace Symfony\Component\Mailer\Transport;

use Symfony\Component\Mailer\Exception\InvalidArgumentException;

/**
 * @author Konstantin Myakshin <molodchick@gmail.com>
 */
final class Dsn
{
    public function __construct(
        private string $scheme,
        private string $host,
        private ?string $user = null,
        #[\SensitiveParameter] private ?string $password = null,
        private ?int $port = null,
        private array $options = [],
    ) {}

    public static function fromString(#[\SensitiveParameter] string $dsn): self
    {}

    public function getScheme(): string
    {}

    public function getHost(): string
    {}

    public function getUser(): ?string
    {}

    public function getPassword(): ?string
    {}

    public function getPort(?int $default = null): ?int
    {}

    public function getOption(string $key, mixed $default = null): mixed
    {}

    public function getBooleanOption(string $key, bool $default = false): bool
    {}
}
