<?php

/*
 * This file is part of the Symfony package.
 *
 * (c) Fabien Potencier <fabien@symfony.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace Symfony\Component\Mime\Encoder;

/**
 * An IDN email address encoder.
 *
 * Encodes the domain part of an address using IDN. This is compatible will all
 * SMTP servers.
 *
 * Note: It leaves the local part as is. In case there are non-ASCII characters
 * in the local part then it depends on the SMTP Server if this is supported.
 *
 * @author Christian Schmidt
 */
final class IdnAddressEncoder implements AddressEncoderInterface
{
    /**
     * Encodes the domain part of an address using IDN.
     */
    public function encodeString(string $address): string
    {}
}
