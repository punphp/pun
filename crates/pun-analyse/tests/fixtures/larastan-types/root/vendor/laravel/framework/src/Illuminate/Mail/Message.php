<?php

namespace Illuminate\Mail;

use Illuminate\Contracts\Mail\Attachable;
use Illuminate\Support\Collection;
use Illuminate\Support\Traits\ForwardsCalls;
use InvalidArgumentException;
use Symfony\Component\Mime\Address;
use Symfony\Component\Mime\Email;
use Symfony\Component\Mime\Part\DataPart;
use Symfony\Component\Mime\Part\File;

/**
 * @mixin \Symfony\Component\Mime\Email
 */
class Message
{
    use ForwardsCalls;

    /**
     * The Symfony Email instance.
     *
     * @var \Symfony\Component\Mime\Email
     */
    protected $message;

    /**
     * CIDs of files embedded in the message.
     *
     * @deprecated Will be removed in a future Laravel version.
     *
     * @var array
     */
    protected $embeddedFiles = [];

    /**
     * Create a new message instance.
     *
     * @param  \Symfony\Component\Mime\Email  $message
     */
    public function __construct(Email $message)
    {}

    /**
     * Add a "from" address to the message.
     *
     * @param  string|array  $address
     * @param  string|null  $name
     * @return $this
     */
    public function from($address, $name = null)
    {}

    /**
     * Set the "sender" of the message.
     *
     * @param  string|array  $address
     * @param  string|null  $name
     * @return $this
     */
    public function sender($address, $name = null)
    {}

    /**
     * Set the "return path" of the message.
     *
     * @param  string  $address
     * @return $this
     */
    public function returnPath($address)
    {}

    /**
     * Add a recipient to the message.
     *
     * @param  string|array  $address
     * @param  string|null  $name
     * @param  bool  $override
     * @return $this
     */
    public function to($address, $name = null, $override = false)
    {}

    /**
     * Remove all "to" addresses from the message.
     *
     * @return $this
     */
    public function forgetTo()
    {}

    /**
     * Add a carbon copy to the message.
     *
     * @param  string|array  $address
     * @param  string|null  $name
     * @param  bool  $override
     * @return $this
     */
    public function cc($address, $name = null, $override = false)
    {}

    /**
     * Remove all carbon copy addresses from the message.
     *
     * @return $this
     */
    public function forgetCc()
    {}

    /**
     * Add a blind carbon copy to the message.
     *
     * @param  string|array  $address
     * @param  string|null  $name
     * @param  bool  $override
     * @return $this
     */
    public function bcc($address, $name = null, $override = false)
    {}

    /**
     * Remove all of the blind carbon copy addresses from the message.
     *
     * @return $this
     */
    public function forgetBcc()
    {}

    /**
     * Add a "reply to" address to the message.
     *
     * @param  string|array  $address
     * @param  string|null  $name
     * @return $this
     */
    public function replyTo($address, $name = null)
    {}

    /**
     * Add a recipient to the message.
     *
     * @param  string|array  $address
     * @param  string|null  $name
     * @param  string  $type
     * @return $this
     */
    protected function addAddresses($address, $name, $type)
    {}

    /**
     * Create a safe Symfony address instance.
     *
     * @param  string  $address
     * @param  string|null  $name
     * @return \Symfony\Component\Mime\Address
     */
    protected function createAddress($address, $name = null)
    {}

    /**
     * Ensure the given address cannot inject additional headers or commands.
     *
     * @param  mixed  $address
     * @return mixed
     */
    protected function ensureAddressIsSafe($address)
    {}

    /**
     * Ensure the given addresses cannot inject additional headers or commands.
     *
     * @param  array  $addresses
     * @return array
     */
    protected function ensureAddressesAreSafe(array $addresses)
    {}

    /**
     * Add an address debug header for a list of recipients.
     *
     * @param  string  $header
     * @param  \Symfony\Component\Mime\Address[]  $addresses
     * @return $this
     */
    protected function addAddressDebugHeader(string $header, array $addresses)
    {}

    /**
     * Set the subject of the message.
     *
     * @param  string  $subject
     * @return $this
     */
    public function subject($subject)
    {}

    /**
     * Set the message priority level.
     *
     * @param  int  $level
     * @return $this
     */
    public function priority($level)
    {}

    /**
     * Attach a file to the message.
     *
     * @param  string|\Illuminate\Contracts\Mail\Attachable|\Illuminate\Mail\Attachment  $file
     * @param  array  $options
     * @return $this
     */
    public function attach($file, array $options = [])
    {}

    /**
     * Attach in-memory data as an attachment.
     *
     * @param  string|resource  $data
     * @param  string  $name
     * @param  array  $options
     * @return $this
     */
    public function attachData($data, $name, array $options = [])
    {}

    /**
     * Embed a file in the message and get the CID.
     *
     * @param  string|\Illuminate\Contracts\Mail\Attachable|\Illuminate\Mail\Attachment  $file
     * @return string
     */
    public function embed($file)
    {}

    /**
     * Embed in-memory data in the message and get the CID.
     *
     * @param  string|resource  $data
     * @param  string  $name
     * @param  string|null  $contentType
     * @return string
     */
    public function embedData($data, $name, $contentType = null)
    {}

    /**
     * Get the underlying Symfony Email instance.
     *
     * @return \Symfony\Component\Mime\Email
     */
    public function getSymfonyMessage()
    {}

    /**
     * Dynamically pass missing methods to the Symfony instance.
     *
     * @param  string  $method
     * @param  array  $parameters
     * @return mixed
     */
    public function __call($method, $parameters)
    {}
}
