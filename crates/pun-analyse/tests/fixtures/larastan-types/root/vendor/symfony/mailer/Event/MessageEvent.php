<?php

/*
 * This file is part of the Symfony package.
 *
 * (c) Fabien Potencier <fabien@symfony.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace Symfony\Component\Mailer\Event;

use Symfony\Component\Mailer\Envelope;
use Symfony\Component\Mailer\Exception\LogicException;
use Symfony\Component\Messenger\Stamp\StampInterface;
use Symfony\Component\Mime\RawMessage;
use Symfony\Contracts\EventDispatcher\Event;

/**
 * Allows the transformation of a Message and the Envelope before the email is sent.
 *
 * @author Fabien Potencier <fabien@symfony.com>
 */
final class MessageEvent extends Event
{
    private bool $rejected = false;

    /** @var StampInterface[] */
    private array $stamps = [];

    public function __construct(
        private RawMessage $message,
        private Envelope $envelope,
        private string $transport,
        private bool $queued = false,
    ) {}

    public function getMessage(): RawMessage
    {}

    public function setMessage(RawMessage $message): void
    {}

    public function getEnvelope(): Envelope
    {}

    public function setEnvelope(Envelope $envelope): void
    {}

    public function getTransport(): string
    {}

    public function isQueued(): bool
    {}

    public function isRejected(): bool
    {}

    public function reject(): void
    {}

    public function addStamp(StampInterface $stamp): void
    {}

    /**
     * @return StampInterface[]
     */
    public function getStamps(): array
    {}
}
