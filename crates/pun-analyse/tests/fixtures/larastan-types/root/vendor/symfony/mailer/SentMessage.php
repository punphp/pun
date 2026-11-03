<?php

/*
 * This file is part of the Symfony package.
 *
 * (c) Fabien Potencier <fabien@symfony.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace Symfony\Component\Mailer;

use Symfony\Component\Mime\Message;
use Symfony\Component\Mime\RawMessage;

/**
 * @author Fabien Potencier <fabien@symfony.com>
 */
class SentMessage
{
    private RawMessage $original;
    private RawMessage $raw;
    private string $messageId;
    private string $debug = '';

    /**
     * @internal
     */
    public function __construct(
        RawMessage $message,
        private Envelope $envelope,
    ) {}

    public function getMessage(): RawMessage
    {}

    public function getOriginalMessage(): RawMessage
    {}

    public function getEnvelope(): Envelope
    {}

    /**
     * Sets the transport-level message ID.
     */
    public function setMessageId(string $id): void
    {}

    /**
     * Gets the transport-level message ID.
     *
     * Not to be confused with the Message-ID header, which is available via getOriginalMessage()
     */
    public function getMessageId(): string
    {}

    public function getDebug(): string
    {}

    public function appendDebug(string $debug): void
    {}

    public function toString(): string
    {}

    public function toIterable(): iterable
    {}
}
