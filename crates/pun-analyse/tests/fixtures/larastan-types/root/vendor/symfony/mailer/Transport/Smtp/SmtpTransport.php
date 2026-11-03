<?php

/*
 * This file is part of the Symfony package.
 *
 * (c) Fabien Potencier <fabien@symfony.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace Symfony\Component\Mailer\Transport\Smtp;

use Psr\EventDispatcher\EventDispatcherInterface;
use Psr\Log\LoggerInterface;
use Symfony\Component\Mailer\Envelope;
use Symfony\Component\Mailer\Exception\InvalidArgumentException;
use Symfony\Component\Mailer\Exception\LogicException;
use Symfony\Component\Mailer\Exception\TransportException;
use Symfony\Component\Mailer\Exception\TransportExceptionInterface;
use Symfony\Component\Mailer\Exception\UnexpectedResponseException;
use Symfony\Component\Mailer\SentMessage;
use Symfony\Component\Mailer\Transport\AbstractTransport;
use Symfony\Component\Mailer\Transport\Smtp\Stream\AbstractStream;
use Symfony\Component\Mailer\Transport\Smtp\Stream\SocketStream;
use Symfony\Component\Mime\RawMessage;

/**
 * Sends emails over SMTP.
 *
 * @author Fabien Potencier <fabien@symfony.com>
 * @author Chris Corbyn
 */
class SmtpTransport extends AbstractTransport
{
    private bool $started = false;
    private int $restartThreshold = 100;
    private int $restartThresholdSleep = 0;
    private int $restartCounter = 0;
    private int $pingThreshold = 100;
    private float $lastMessageTime = 0;
    private AbstractStream $stream;
    private string $domain = '[127.0.0.1]';

    public function __construct(?AbstractStream $stream = null, ?EventDispatcherInterface $dispatcher = null, ?LoggerInterface $logger = null)
    {}

    public function getStream(): AbstractStream
    {}

    /**
     * Sets the maximum number of messages to send before re-starting the transport.
     *
     * By default, the threshold is set to 100 (and no sleep at restart).
     *
     * @param int $threshold The maximum number of messages (0 to disable)
     * @param int $sleep     The number of seconds to sleep between stopping and re-starting the transport
     *
     * @return $this
     */
    public function setRestartThreshold(int $threshold, int $sleep = 0): static
    {}

    /**
     * Sets the minimum number of seconds required between two messages, before the server is pinged.
     * If the transport wants to send a message and the time since the last message exceeds the specified threshold,
     * the transport will ping the server first (NOOP command) to check if the connection is still alive.
     * Otherwise the message will be sent without pinging the server first.
     *
     * Do not set the threshold too low, as the SMTP server may drop the connection if there are too many
     * non-mail commands (like pinging the server with NOOP).
     *
     * By default, the threshold is set to 100 seconds.
     *
     * @param int $seconds The minimum number of seconds between two messages required to ping the server
     *
     * @return $this
     */
    public function setPingThreshold(int $seconds): static
    {}

    /**
     * Sets the name of the local domain that will be used in HELO.
     *
     * This should be a fully-qualified domain name and should be truly the domain
     * you're using.
     *
     * If your server does not have a domain name, use the IP address. This will
     * automatically be wrapped in square brackets as described in RFC 5321,
     * section 4.1.3.
     *
     * @return $this
     */
    public function setLocalDomain(string $domain): static
    {}

    /**
     * Gets the name of the domain that will be used in HELO.
     *
     * If an IP address was specified, this will be returned wrapped in square
     * brackets as described in RFC 5321, section 4.1.3.
     */
    public function getLocalDomain(): string
    {}

    public function send(RawMessage $message, ?Envelope $envelope = null): ?SentMessage
    {}

    protected function parseMessageId(string $mtaResult): string
    {}

    public function __toString(): string
    {}

    /**
     * Runs a command against the stream, expecting the given response codes.
     *
     * @param int[] $codes
     *
     * @throws TransportException when an invalid response if received
     */
    public function executeCommand(string $command, array $codes): string
    {}

    protected function doSend(SentMessage $message): void
    {}

    protected function serverSupportsSmtpUtf8(): bool
    {}

    private function doHeloCommand(): void
    {}

    private function doMailFromCommand(string $address, bool $smtputf8): void
    {}

    private function doRcptToCommand(string $address): void
    {}

    public function start(): void
    {}

    /**
     * Manually disconnect from the SMTP server.
     *
     * In most cases this is not necessary since the disconnect happens automatically on termination.
     * In cases of long-running scripts, this might however make sense to avoid keeping an open
     * connection to the SMTP server in between sending emails.
     */
    public function stop(): void
    {}

    private function ping(): void
    {}

    /**
     * @throws TransportException if a response code is incorrect
     */
    private function assertResponseCode(string $response, array $codes): void
    {}

    private function getFullResponse(): string
    {}

    private function checkRestartThreshold(): void
    {}

    public function __serialize(): array
    {}

    public function __unserialize(array $data): void
    {}

    public function __destruct()
    {}
}
