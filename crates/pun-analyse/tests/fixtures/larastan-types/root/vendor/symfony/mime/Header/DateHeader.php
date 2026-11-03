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

/**
 * A Date MIME Header.
 *
 * @author Chris Corbyn
 */
final class DateHeader extends AbstractHeader
{
    private \DateTimeImmutable $dateTime;

    public function __construct(string $name, \DateTimeInterface $date)
    {}

    /**
     * @param \DateTimeInterface $body
     */
    public function setBody(mixed $body): void
    {}

    public function getBody(): \DateTimeImmutable
    {}

    public function getDateTime(): \DateTimeImmutable
    {}

    /**
     * Set the date-time of the Date in this Header.
     *
     * If a DateTime instance is provided, it is converted to DateTimeImmutable.
     */
    public function setDateTime(\DateTimeInterface $dateTime): void
    {}

    public function getBodyAsString(): string
    {}
}
