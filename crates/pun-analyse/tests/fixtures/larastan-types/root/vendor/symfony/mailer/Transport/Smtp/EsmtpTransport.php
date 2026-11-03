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
use Symfony\Component\Mailer\Exception\TransportException;
use Symfony\Component\Mailer\Exception\TransportExceptionInterface;
use Symfony\Component\Mailer\Exception\UnexpectedResponseException;
use Symfony\Component\Mailer\Transport\Smtp\Auth\AuthenticatorInterface;
use Symfony\Component\Mailer\Transport\Smtp\Stream\AbstractStream;
use Symfony\Component\Mailer\Transport\Smtp\Stream\SocketStream;

/**
 * Sends Emails over SMTP with ESMTP support.
 *
 * @author Fabien Potencier <fabien@symfony.com>
 * @author Chris Corbyn
 */
class EsmtpTransport extends SmtpTransport
{
    private array $authenticators = [];
    private string $username = '';
    private string $password = '';
    private array $capabilities;
    private bool $autoTls = true;
    private bool $requireTls = false;

    public function __construct(string $host = 'localhost', int $port = 0, ?bool $tls = null, ?EventDispatcherInterface $dispatcher = null, ?LoggerInterface $logger = null, ?AbstractStream $stream = null, ?array $authenticators = null)
    {}

    /**
     * @return $this
     */
    public function setUsername(string $username): static
    {}

    public function getUsername(): string
    {}

    /**
     * @return $this
     */
    public function setPassword(#[\SensitiveParameter] string $password): static
    {}

    public function getPassword(): string
    {}

    /**
     * @return $this
     */
    public function setAutoTls(bool $autoTls): static
    {}

    public function isAutoTls(): bool
    {}

    /**
     * @return $this
     */
    public function setRequireTls(bool $requireTls): static
    {}

    public function isTlsRequired(): bool
    {}

    public function setAuthenticators(array $authenticators): void
    {}

    public function addAuthenticator(AuthenticatorInterface $authenticator): void
    {}

    public function executeCommand(string $command, array $codes): string
    {}

    final protected function getCapabilities(): array
    {}

    private function doEhloCommand(): string
    {}

    private function parseCapabilities(string $ehloResponse): array
    {}

    protected function serverSupportsSmtpUtf8(): bool
    {}

    private function handleAuth(array $modes): void
    {}
}
