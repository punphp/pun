<?php

namespace Illuminate\Mail\Transport;

use Exception;
use SensitiveParameter;
use Symfony\Component\HttpClient\HttpClient;
use Symfony\Component\HttpFoundation\Response;
use Symfony\Component\Mailer\Envelope;
use Symfony\Component\Mailer\Exception\TransportException;
use Symfony\Component\Mailer\SentMessage;
use Symfony\Component\Mailer\Transport\AbstractTransport;
use Symfony\Component\Mime\Address;
use Symfony\Component\Mime\Email;
use Symfony\Component\Mime\MessageConverter;
use Symfony\Contracts\HttpClient\HttpClientInterface;

class CloudflareTransport extends AbstractTransport
{
    /**
     * The HTTP Client instance.
     */
    protected HttpClientInterface $client;

    /**
     * Create a new Cloudflare transport instance.
     */
    public function __construct(
        protected string $accountId,
        #[SensitiveParameter] protected string $key,
        ?HttpClientInterface $client = null,
    ) {}

    /**
     * {@inheritDoc}
     *
     * @throws TransportException
     */
    protected function doSend(SentMessage $message): void
    {}

    /**
     * Get the Cloudflare payload for the given message.
     */
    protected function getPayload(SentMessage $message): array
    {}

    /**
     * Get the recipients without CC or BCC.
     */
    protected function getRecipients(Email $email, Envelope $envelope): array
    {}

    /**
     * Get the custom headers for the email, excluding the standard ones.
     */
    protected function getCustomHeaders(Email $email): array
    {}

    /**
     * Get the attachments formatted for the Cloudflare API.
     */
    protected function getAttachments(Email $email): array
    {}

    /**
     * Get the address formatted for the Cloudflare API.
     *
     * @return string|array
     */
    protected function formatAddress(Address $address)
    {}

    /**
     * Get multiple addresses formatted as strings for the Cloudflare API.
     */
    protected function stringifyAddresses(array $addresses): array
    {}

    /**
     * Get the string representation of the transport.
     */
    public function __toString(): string
    {}
}
