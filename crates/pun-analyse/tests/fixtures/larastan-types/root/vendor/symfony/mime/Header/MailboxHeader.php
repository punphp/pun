<?php

/*
 * This file is part of the Symfony package.
 *
 * (c) Fabien Potencier <fabien@symfony.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace Symfony\Component\Mime\Header;

use Symfony\Component\Mime\Address;
use Symfony\Component\Mime\Exception\RfcComplianceException;

/**
 * A Mailbox MIME Header for something like Sender (one named address).
 *
 * @author Fabien Potencier <fabien@symfony.com>
 */
final class MailboxHeader extends AbstractHeader
{
    private Address $address;

    public function __construct(string $name, Address $address)
    {}

    /**
     * @param Address $body
     *
     * @throws RfcComplianceException
     */
    public function setBody(mixed $body): void
    {}

    /**
     * @throws RfcComplianceException
     */
    public function getBody(): Address
    {}

    /**
     * @throws RfcComplianceException
     */
    public function setAddress(Address $address): void
    {}

    public function getAddress(): Address
    {}

    public function getBodyAsString(): string
    {}

    /**
     * Redefine the encoding requirements for an address.
     *
     * All "specials" must be encoded as the full header value will not be quoted
     *
     * @see RFC 2822 3.2.1
     */
    protected function tokenNeedsEncoding(string $token): bool
    {}
}
