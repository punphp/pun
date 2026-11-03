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
 * A Mailbox list MIME Header for something like From, To, Cc, and Bcc (one or more named addresses).
 *
 * @author Chris Corbyn
 */
final class MailboxListHeader extends AbstractHeader
{
    private array $addresses = [];

    /**
     * @param Address[] $addresses
     */
    public function __construct(string $name, array $addresses)
    {}

    /**
     * @param Address[] $body
     *
     * @throws RfcComplianceException
     */
    public function setBody(mixed $body): void
    {}

    /**
     * @return Address[]
     *
     * @throws RfcComplianceException
     */
    public function getBody(): array
    {}

    /**
     * Sets a list of addresses to be shown in this Header.
     *
     * @param Address[] $addresses
     *
     * @throws RfcComplianceException
     */
    public function setAddresses(array $addresses): void
    {}

    /**
     * Sets a list of addresses to be shown in this Header.
     *
     * @param Address[] $addresses
     *
     * @throws RfcComplianceException
     */
    public function addAddresses(array $addresses): void
    {}

    /**
     * @throws RfcComplianceException
     */
    public function addAddress(Address $address): void
    {}

    /**
     * @return Address[]
     */
    public function getAddresses(): array
    {}

    /**
     * Gets the full mailbox list of this Header as an array of valid RFC 2822 strings.
     *
     * @return string[]
     *
     * @throws RfcComplianceException
     */
    public function getAddressStrings(): array
    {}

    public function getBodyAsString(): string
    {}

    /**
     * Redefine the encoding requirements for addresses.
     *
     * All "specials" must be encoded as the full header value will not be quoted
     *
     * @see RFC 2822 3.2.1
     */
    protected function tokenNeedsEncoding(string $token): bool
    {}
}
