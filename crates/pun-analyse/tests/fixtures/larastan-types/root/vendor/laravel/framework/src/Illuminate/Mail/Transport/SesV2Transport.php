<?php

namespace Illuminate\Mail\Transport;

use Aws\Exception\AwsException;
use Aws\SesV2\SesV2Client;
use Illuminate\Support\Collection;
use Stringable;
use Symfony\Component\Mailer\Exception\TransportException;
use Symfony\Component\Mailer\Header\MetadataHeader;
use Symfony\Component\Mailer\SentMessage;
use Symfony\Component\Mailer\Transport\AbstractTransport;
use Symfony\Component\Mime\Message;

class SesV2Transport extends AbstractTransport implements Stringable
{
    /**
     * The Amazon SES V2 instance.
     *
     * @var \Aws\SesV2\SesV2Client
     */
    protected $ses;

    /**
     * The Amazon SES transmission options.
     *
     * @var array
     */
    protected $options = [];

    /**
     * Create a new SES V2 transport instance.
     *
     * @param  \Aws\SesV2\SesV2Client  $ses
     * @param  array  $options
     */
    public function __construct(SesV2Client $ses, $options = [])
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
     * Extract the SES tenant name, if applicable.
     *
     * @param  \Symfony\Component\Mailer\SentMessage  $message
     * @return string|null
     */
    protected function tenantName(SentMessage $message)
    {}

    /**
     * Get the Amazon SES V2 client for the SesV2Transport instance.
     *
     * @return \Aws\SesV2\SesV2Client
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
