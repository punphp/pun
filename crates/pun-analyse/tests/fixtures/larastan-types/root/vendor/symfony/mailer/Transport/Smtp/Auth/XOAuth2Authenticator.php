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

use Symfony\Component\Mailer\Transport\Smtp\EsmtpTransport;

/**
 * Handles XOAUTH2 authentication.
 *
 * @author xu.li<AthenaLightenedMyPath@gmail.com>
 *
 * @see https://developers.google.com/google-apps/gmail/xoauth2_protocol
 */
class XOAuth2Authenticator implements AuthenticatorInterface
{
    public function getAuthKeyword(): string
    {}

    /**
     * @see https://developers.google.com/google-apps/gmail/xoauth2_protocol#the_sasl_xoauth2_mechanism
     */
    public function authenticate(EsmtpTransport $client): void
    {}
}
