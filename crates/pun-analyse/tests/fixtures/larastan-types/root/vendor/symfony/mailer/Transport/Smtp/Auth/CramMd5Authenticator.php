<?php

/*
 * This file is part of the Symfony package.
 *
 * (c) Fabien Potencier <fabien@symfony.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace Symfony\Component\Mailer\Transport\Smtp\Auth;

use Symfony\Component\Mailer\Exception\InvalidArgumentException;
use Symfony\Component\Mailer\Transport\Smtp\EsmtpTransport;

/**
 * Handles CRAM-MD5 authentication.
 *
 * @author Chris Corbyn
 */
class CramMd5Authenticator implements AuthenticatorInterface
{
    public function getAuthKeyword(): string
    {}

    /**
     * @see https://www.ietf.org/rfc/rfc4954.txt
     */
    public function authenticate(EsmtpTransport $client): void
    {}

    /**
     * Generates a CRAM-MD5 response from a server challenge.
     */
    private function getResponse(#[\SensitiveParameter] string $secret, string $challenge): string
    {}
}
