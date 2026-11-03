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

use Symfony\Component\Mailer\Exception\TransportException;
use Symfony\Component\Mailer\Exception\UnsupportedSchemeException;
use Symfony\Component\Mailer\Transport\Smtp\SmtpTransport;
use Symfony\Component\Mailer\Transport\Smtp\Stream\SocketStream;

/**
 * Factory that configures a transport (sendmail or SMTP) based on php.ini settings.
 *
 * @author Laurent VOULLEMIER <laurent.voullemier@gmail.com>
 */
final class NativeTransportFactory extends AbstractTransportFactory
{
    public function create(Dsn $dsn): TransportInterface
    {}

    protected function getSupportedSchemes(): array
    {}
}

// @php-cs-fixer-ignore native_function_invocation As we explicitly break it for testability reason, ref https://github.com/symfony/symfony/pull/59195
