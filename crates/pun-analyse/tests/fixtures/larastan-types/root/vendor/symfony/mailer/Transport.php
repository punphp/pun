<?php

/*
 * This file is part of the Symfony package.
 *
 * (c) Fabien Potencier <fabien@symfony.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace Symfony\Component\Mailer;

use Psr\EventDispatcher\EventDispatcherInterface;
use Psr\Log\LoggerInterface;
use Symfony\Component\Mailer\Bridge\AhaSend\Transport\AhaSendTransportFactory;
use Symfony\Component\Mailer\Bridge\Amazon\Transport\SesTransportFactory;
use Symfony\Component\Mailer\Bridge\Azure\Transport\AzureTransportFactory;
use Symfony\Component\Mailer\Bridge\Brevo\Transport\BrevoTransportFactory;
use Symfony\Component\Mailer\Bridge\Google\Transport\GmailTransportFactory;
use Symfony\Component\Mailer\Bridge\Infobip\Transport\InfobipTransportFactory;
use Symfony\Component\Mailer\Bridge\Mailchimp\Transport\MandrillTransportFactory;
use Symfony\Component\Mailer\Bridge\MailerSend\Transport\MailerSendTransportFactory;
use Symfony\Component\Mailer\Bridge\Mailgun\Transport\MailgunTransportFactory;
use Symfony\Component\Mailer\Bridge\Mailjet\Transport\MailjetTransportFactory;
use Symfony\Component\Mailer\Bridge\Mailomat\Transport\MailomatTransportFactory;
use Symfony\Component\Mailer\Bridge\MailPace\Transport\MailPaceTransportFactory;
use Symfony\Component\Mailer\Bridge\Mailtrap\Transport\MailtrapTransportFactory;
use Symfony\Component\Mailer\Bridge\MicrosoftGraph\Transport\MicrosoftGraphTransportFactory;
use Symfony\Component\Mailer\Bridge\Postal\Transport\PostalTransportFactory;
use Symfony\Component\Mailer\Bridge\Postmark\Transport\PostmarkTransportFactory;
use Symfony\Component\Mailer\Bridge\Resend\Transport\ResendTransportFactory;
use Symfony\Component\Mailer\Bridge\Scaleway\Transport\ScalewayTransportFactory;
use Symfony\Component\Mailer\Bridge\Sendgrid\Transport\SendgridTransportFactory;
use Symfony\Component\Mailer\Bridge\Sweego\Transport\SweegoTransportFactory;
use Symfony\Component\Mailer\Exception\InvalidArgumentException;
use Symfony\Component\Mailer\Exception\UnsupportedSchemeException;
use Symfony\Component\Mailer\Transport\Dsn;
use Symfony\Component\Mailer\Transport\FailoverTransport;
use Symfony\Component\Mailer\Transport\NativeTransportFactory;
use Symfony\Component\Mailer\Transport\NullTransportFactory;
use Symfony\Component\Mailer\Transport\RoundRobinTransport;
use Symfony\Component\Mailer\Transport\SendmailTransportFactory;
use Symfony\Component\Mailer\Transport\Smtp\EsmtpTransportFactory;
use Symfony\Component\Mailer\Transport\TransportFactoryInterface;
use Symfony\Component\Mailer\Transport\TransportInterface;
use Symfony\Component\Mailer\Transport\Transports;
use Symfony\Contracts\HttpClient\HttpClientInterface;

/**
 * @author Fabien Potencier <fabien@symfony.com>
 * @author Konstantin Myakshin <molodchick@gmail.com>
 */
final class Transport
{
    private const FACTORY_CLASSES = [
        AhaSendTransportFactory::class,
        AzureTransportFactory::class,
        BrevoTransportFactory::class,
        GmailTransportFactory::class,
        InfobipTransportFactory::class,
        MailerSendTransportFactory::class,
        MailgunTransportFactory::class,
        MailjetTransportFactory::class,
        MailomatTransportFactory::class,
        MailPaceTransportFactory::class,
        MandrillTransportFactory::class,
        PostalTransportFactory::class,
        PostmarkTransportFactory::class,
        MailtrapTransportFactory::class,
        MicrosoftGraphTransportFactory::class,
        ResendTransportFactory::class,
        ScalewayTransportFactory::class,
        SendgridTransportFactory::class,
        SesTransportFactory::class,
        SweegoTransportFactory::class,
    ];

    public static function fromDsn(#[\SensitiveParameter] string $dsn, ?EventDispatcherInterface $dispatcher = null, ?HttpClientInterface $client = null, ?LoggerInterface $logger = null): TransportInterface
    {}

    public static function fromDsns(#[\SensitiveParameter] array $dsns, ?EventDispatcherInterface $dispatcher = null, ?HttpClientInterface $client = null, ?LoggerInterface $logger = null): TransportInterface
    {}

    /**
     * @param TransportFactoryInterface[] $factories
     */
    public function __construct(
        private iterable $factories,
    ) {}

    public function fromStrings(#[\SensitiveParameter] array $dsns): Transports
    {}

    public function fromString(#[\SensitiveParameter] string $dsn): TransportInterface
    {}

    private function parseDsn(#[\SensitiveParameter] string $dsn, int $offset = 0): array
    {}

    public function fromDsnObject(Dsn $dsn): TransportInterface
    {}

    /**
     * @return \Traversable<int, TransportFactoryInterface>
     */
    public static function getDefaultFactories(?EventDispatcherInterface $dispatcher = null, ?HttpClientInterface $client = null, ?LoggerInterface $logger = null): \Traversable
    { yield; }
}
