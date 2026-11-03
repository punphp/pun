<?php

namespace Illuminate\Mail\Transport;

use Aws\Exception\AwsException;
use Aws\Ses\SesClient;
use Illuminate\Support\Collection;
use Stringable;
use Symfony\Component\Mailer\Exception\TransportException;
use Symfony\Component\Mailer\Header\MetadataHeader;
use Symfony\Component\Mailer\SentMessage;
use Symfony\Component\Mailer\Transport\AbstractTransport;
use Symfony\Component\Mime\Message;

class SesTransport extends AbstractTransport implements Stringable
{
    /**
     * The Amazon SES instance.
     *
     * @var \Aws\Ses\SesClient
     */
    protected $ses;

    /**
     * The Amazon SES transmission options.
     *
     * @var array
     */
    protected $options = [];

    /**
     * Create a new SES transport instance.
     *
     * @param  \Aws\Ses\SesClient  $ses
     * @param  array  $options
     */
    public function __construct(SesClient $ses, $options = [])
    {}

    /**
     * {@inheritDoc}
     *
     * @throws \Symfony\Component\Mailer\Exception\TransportException
     */
    protected function doSend(SentMessage $message): void
    {}

    /**
     * Extract the SES list management options, if applicable.
     *
     * @param  \Symfony\Component\Mailer\SentMessage  $message
     * @return array|null
     */
    protected function listManagementOptions(SentMessage $message)
    {}

    /**
     * Get the Amazon SES client for the SesTransport instance.
     *
     * @return \Aws\Ses\SesClient
     */
    public function ses()
    {}

    /**
     * Get the transmission options being used by the transport.
     *
     * @return array
     */
    public function getOptions()
    {}

    /**
     * Set the transmission options being used by the transport.
     *
     * @param  array  $options
     * @return array
     */
    public function setOptions(array $options)
    {}

    /**
     * Get the string representation of the transport.
     *
     * @return string
     */
    public function __toString(): string
    {}
}
