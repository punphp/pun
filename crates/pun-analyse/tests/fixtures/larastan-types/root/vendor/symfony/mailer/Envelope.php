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

use Symfony\Component\Mailer\Exception\InvalidArgumentException;
use Symfony\Component\Mailer\Exception\LogicException;
use Symfony\Component\Mime\Address;
use Symfony\Component\Mime\RawMessage;

/**
 * @author Fabien Potencier <fabien@symfony.com>
 */
class Envelope
{
    private Address $sender;
    private array $recipients = [];

    /**
     * @param Address[] $recipients
     */
    public function __construct(Address $sender, array $recipients)
    {}

    public static function create(RawMessage $message): self
    {}

    public function setSender(Address $sender): void
    {}

    /**
     * @return Address Returns a "mailbox" as specified by RFC 2822
     *                 Must be converted to an "addr-spec" when used as a "MAIL FROM" value in SMTP (use getAddress())
     */
    public function getSender(): Address
    {}

    /**
     * @param Address[] $recipients
     */
    public function setRecipients(array $recipients): void
    {}

    /**
     * @return Address[]
     */
    public function getRecipients(): array
    {}

    /**
     * Returns true if any address' localpart contains at least one
     * non-ASCII character, and false if all addresses have all-ASCII
     * localparts.
     *
     * This helps to decide whether to the SMTPUTF8 extensions (RFC
     * 6530 and following) for any given message.
     *
     * The SMTPUTF8 extension is strictly required if any address
     * contains a non-ASCII character in its localpart. If non-ASCII
     * is only used in domains (e.g. horst@freiherr-von-mühlhausen.de)
     * then it is possible to send the message using IDN encoding
     * instead of SMTPUTF8. The most common software will display the
     * message as intended.
     */
    public function anyAddressHasUnicodeLocalpart(): bool
    {}
}
