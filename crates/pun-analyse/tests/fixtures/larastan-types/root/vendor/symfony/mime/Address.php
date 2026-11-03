<?php

/*
 * This file is part of the Symfony package.
 *
 * (c) Fabien Potencier <fabien@symfony.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace Symfony\Component\Mime;

use Egulias\EmailValidator\EmailValidator;
use Egulias\EmailValidator\Validation\MessageIDValidation;
use Egulias\EmailValidator\Validation\RFCValidation;
use Symfony\Component\Mime\Encoder\IdnAddressEncoder;
use Symfony\Component\Mime\Exception\InvalidArgumentException;
use Symfony\Component\Mime\Exception\LogicException;
use Symfony\Component\Mime\Exception\RfcComplianceException;

/**
 * @author Fabien Potencier <fabien@symfony.com>
 */
final class Address
{
    /**
     * A regex that matches a structure like 'Name <email@address.com>'.
     * It matches anything between the first < and last > as email address.
     * This allows to use a single string to construct an Address, which can be convenient to use in
     * config, and allows to have more readable config.
     * This does not try to cover all edge cases for address.
     */
    private const FROM_STRING_PATTERN = '~(?<displayName>[^<]*)<(?<addrSpec>.*)>[^>]*~';

    private static EmailValidator $validator;
    private static IdnAddressEncoder $encoder;

    private string $address;
    private string $name;

    public function __construct(string $address, string $name = '')
    {}

    public function getAddress(): string
    {}

    public function getName(): string
    {}

    public function getEncodedAddress(): string
    {}

    public function toString(): string
    {}

    public function getEncodedName(): string
    {}

    public static function create(self|string $address): self
    {}

    /**
     * @param array<Address|string> $addresses
     *
     * @return Address[]
     */
    public static function createArray(array $addresses): array
    {}

    /**
     * Returns true if this address' localpart contains at least one
     * non-ASCII character, and false if it is only ASCII (or empty).
     *
     * This is a helper for Envelope, which has to decide whether to
     * the SMTPUTF8 extensions (RFC 6530 and following) for any given
     * message.
     *
     * The SMTPUTF8 extension is strictly required if any address
     * contains a non-ASCII character in its localpart. If non-ASCII
     * is only used in domains (e.g. horst@freiherr-von-mühlhausen.de)
     * then it is possible to send the message using IDN encoding
     * instead of SMTPUTF8. The most common software will display the
     * message as intended.
     */
    public function hasUnicodeLocalpart(): bool
    {}

    private static function isValidAddrSpec(string $address): bool
    {}
}
