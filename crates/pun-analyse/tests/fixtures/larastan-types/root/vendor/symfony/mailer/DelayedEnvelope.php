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

use Symfony\Component\Mailer\Exception\LogicException;
use Symfony\Component\Mime\Address;
use Symfony\Component\Mime\Header\Headers;
use Symfony\Component\Mime\Message;

/**
 * @author Fabien Potencier <fabien@symfony.com>
 *
 * @internal
 */
final class DelayedEnvelope extends Envelope
{
    private bool $senderSet = false;
    private bool $recipientsSet = false;

    public function __construct(
        private Message $message,
    ) {}

    public function setSender(Address $sender): void
    {}

    public function getSender(): Address
    {}

    public function setRecipients(array $recipients): void
    {}

    /**
     * @return Address[]
     */
    public function getRecipients(): array
    {}

    private static function getRecipientsFromHeaders(Headers $headers): array
    {}

    private static function getSenderFromHeaders(Headers $headers): Address
    {}
}
