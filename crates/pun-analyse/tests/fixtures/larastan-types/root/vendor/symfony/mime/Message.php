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

use Symfony\Component\Mime\Exception\LogicException;
use Symfony\Component\Mime\Header\Headers;
use Symfony\Component\Mime\Part\AbstractPart;
use Symfony\Component\Mime\Part\TextPart;

/**
 * @author Fabien Potencier <fabien@symfony.com>
 */
class Message extends RawMessage
{
    private Headers $headers;

    public function __construct(
        ?Headers $headers = null,
        private ?AbstractPart $body = null,
    ) {}

    public function __clone()
    {}

    /**
     * @return $this
     */
    public function setBody(?AbstractPart $body): static
    {}

    public function getBody(): ?AbstractPart
    {}

    /**
     * @return $this
     */
    public function setHeaders(Headers $headers): static
    {}

    public function getHeaders(): Headers
    {}

    public function getPreparedHeaders(): Headers
    {}

    public function toString(): string
    {}

    public function toIterable(): iterable
    { yield; }

    public function ensureValidity(): void
    {}

    public function generateMessageId(): string
    {}

    public function __serialize(): array
    {}

    public function __unserialize(array $data): void
    {}
}
