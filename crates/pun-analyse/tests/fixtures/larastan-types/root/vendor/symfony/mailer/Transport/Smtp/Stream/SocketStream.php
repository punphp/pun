<?php

/*
 * This file is part of the Symfony package.
 *
 * (c) Fabien Potencier <fabien@symfony.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace Symfony\Component\Mailer\Transport\Smtp\Stream;

use Symfony\Component\Mailer\Exception\TransportException;

/**
 * A stream supporting remote sockets.
 *
 * @author Fabien Potencier <fabien@symfony.com>
 * @author Chris Corbyn
 *
 * @internal
 */
final class SocketStream extends AbstractStream
{
    private string $url;
    private string $host = 'localhost';
    private int $port = 465;
    private float $timeout;
    private bool $tls = true;
    private ?string $sourceIp = null;
    private array $streamContextOptions = [];

    /**
     * @return $this
     */
    public function setTimeout(float $timeout): static
    {}

    public function getTimeout(): float
    {}

    /**
     * Literal IPv6 addresses should be wrapped in square brackets.
     *
     * @return $this
     */
    public function setHost(string $host): static
    {}

    public function getHost(): string
    {}

    /**
     * @return $this
     */
    public function setPort(int $port): static
    {}

    public function getPort(): int
    {}

    /**
     * Sets the TLS/SSL on the socket (disables STARTTLS).
     *
     * @return $this
     */
    public function disableTls(): static
    {}

    public function isTLS(): bool
    {}

    /**
     * @return $this
     */
    public function setStreamOptions(array $options): static
    {}

    public function getStreamOptions(): array
    {}

    /**
     * Sets the source IP.
     *
     * IPv6 addresses should be wrapped in square brackets.
     *
     * @return $this
     */
    public function setSourceIp(string $ip): static
    {}

    /**
     * Returns the IP used to connect to the destination.
     */
    public function getSourceIp(): ?string
    {}

    public function initialize(): void
    {}

    public function startTLS(): bool
    {}

    public function terminate(): void
    {}

    protected function getReadConnectionDescription(): string
    {}
}
