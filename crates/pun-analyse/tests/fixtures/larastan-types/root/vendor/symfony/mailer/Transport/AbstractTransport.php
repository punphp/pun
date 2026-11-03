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

use Psr\EventDispatcher\EventDispatcherInterface;
use Psr\Log\LoggerInterface;
use Psr\Log\NullLogger;
use Symfony\Bridge\Twig\Mime\TemplatedEmail;
use Symfony\Component\Mailer\Envelope;
use Symfony\Component\Mailer\Event\FailedMessageEvent;
use Symfony\Component\Mailer\Event\MessageEvent;
use Symfony\Component\Mailer\Event\SentMessageEvent;
use Symfony\Component\Mailer\Exception\LogicException;
use Symfony\Component\Mailer\SentMessage;
use Symfony\Component\Mime\Address;
use Symfony\Component\Mime\BodyRendererInterface;
use Symfony\Component\Mime\RawMessage;

/**
 * @author Fabien Potencier <fabien@symfony.com>
 */
abstract class AbstractTransport implements TransportInterface
{
    private LoggerInterface $logger;
    private float $rate = 0;
    private float $lastSent = 0;

    public function __construct(
        private ?EventDispatcherInterface $dispatcher = null,
        ?LoggerInterface $logger = null,
    ) {}

    /**
     * Sets the maximum number of messages to send per second (0 to disable).
     *
     * @return $this
     */
    public function setMaxPerSecond(float $rate): static
    {}

    public function send(RawMessage $message, ?Envelope $envelope = null): ?SentMessage
    {}

    abstract protected function doSend(SentMessage $message): void;

    /**
     * @param Address[] $addresses
     *
     * @return string[]
     */
    protected function stringifyAddresses(array $addresses): array
    {}

    protected function getLogger(): LoggerInterface
    {}

    private function checkThrottling(): void
    {}
}
