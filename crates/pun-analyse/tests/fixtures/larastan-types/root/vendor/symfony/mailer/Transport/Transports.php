<?php

/*
 * This file is part of the Symfony package.
 *
 * (c) Fabien Potencier <fabien@symfony.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace Symfony\Component\Mailer\Transport;

use Symfony\Component\Mailer\Envelope;
use Symfony\Component\Mailer\Exception\InvalidArgumentException;
use Symfony\Component\Mailer\Exception\LogicException;
use Symfony\Component\Mailer\SentMessage;
use Symfony\Component\Mime\Message;
use Symfony\Component\Mime\RawMessage;

/**
 * @author Fabien Potencier <fabien@symfony.com>
 */
final class Transports implements TransportInterface
{
    /**
     * @var array<string, TransportInterface>
     */
    private array $transports = [];
    private TransportInterface $default;

    /**
     * @param iterable<string, TransportInterface> $transports
     */
    public function __construct(iterable $transports)
    {}

    public function send(RawMessage $message, ?Envelope $envelope = null): ?SentMessage
    {}

    public function __toString(): string
    {}
}
