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

/**
 * @author Fabien Potencier <fabien@symfony.com>
 */
class RawMessage
{
    private bool $isGeneratorClosed;

    /**
     * @param iterable<string>|string|resource $message
     */
    public function __construct(
        private $message,
    ) {}

    public function __destruct()
    {}

    public function toString(): string
    {}

    public function toIterable(): iterable
    { yield; }

    /**
     * @throws LogicException if the message is not valid
     */
    public function ensureValidity(): void
    {}

    public function __serialize(): array
    {}

    public function __unserialize(array $data): void
    {}
}
