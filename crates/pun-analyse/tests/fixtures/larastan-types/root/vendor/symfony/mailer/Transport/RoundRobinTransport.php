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

use Psr\Log\LoggerInterface;
use Psr\Log\NullLogger;
use Symfony\Component\Mailer\Envelope;
use Symfony\Component\Mailer\Exception\TransportException;
use Symfony\Component\Mailer\Exception\TransportExceptionInterface;
use Symfony\Component\Mailer\SentMessage;
use Symfony\Component\Mime\RawMessage;

/**
 * Uses several Transports using a round robin algorithm.
 *
 * @author Fabien Potencier <fabien@symfony.com>
 */
class RoundRobinTransport implements TransportInterface
{
    /**
     * @var \SplObjectStorage<TransportInterface, float>
     */
    private \SplObjectStorage $deadTransports;
    private int $cursor = -1;

    /**
     * @param TransportInterface[] $transports
     */
    public function __construct(
        private array $transports,
        private int $retryPeriod = 60,
        private LoggerInterface $logger = new NullLogger(),
    ) {}

    public function send(RawMessage $message, ?Envelope $envelope = null): ?SentMessage
    {}

    public function __toString(): string
    {}

    /**
     * Rotates the transport list around and returns the first instance.
     */
    protected function getNextTransport(): ?TransportInterface
    {}

    protected function isTransportDead(TransportInterface $transport): bool
    {}

    protected function getInitialCursor(): int
    {}

    protected function getNameSymbol(): string
    {}

    private function moveCursor(int $cursor): int
    {}
}
