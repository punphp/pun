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
 * An ID MIME Header for something like Message-ID or Content-ID (one or more addresses).
 *
 * @author Chris Corbyn
 */
final class IdentificationHeader extends AbstractHeader
{
    private array $ids = [];
    private array $idsAsAddresses = [];

    public function __construct(string $name, string|array $ids)
    {}

    /**
     * @param string|string[] $body a string ID or an array of IDs
     *
     * @throws RfcComplianceException
     */
    public function setBody(mixed $body): void
    {}

    public function getBody(): array
    {}

    /**
     * Set the ID used in the value of this header.
     *
     * @param string|string[] $id
     *
     * @throws RfcComplianceException
     */
    public function setId(string|array $id): void
    {}

    /**
     * Get the ID used in the value of this Header.
     *
     * If multiple IDs are set only the first is returned.
     */
    public function getId(): ?string
    {}

    /**
     * Set a collection of IDs to use in the value of this Header.
     *
     * @param string[] $ids
     *
     * @throws RfcComplianceException
     */
    public function setIds(array $ids): void
    {}

    /**
     * Get the list of IDs used in this Header.
     *
     * @return string[]
     */
    public function getIds(): array
    {}

    public function getBodyAsString(): string
    {}
}
