<?php

namespace Illuminate\Mail\Events;

use Exception;
use Illuminate\Mail\SentMessage;
use Illuminate\Support\Collection;

/**
 * @property \Symfony\Component\Mime\Email $message
 */
class MessageSent
{
    /**
     * Create a new event instance.
     *
     * @param  \Illuminate\Mail\SentMessage  $sent  The message that was sent.
     * @param  array  $data  The message data.
     */
    public function __construct(
        public SentMessage $sent,
        public array $data = [],
    ) {}

    /**
     * Get the serializable representation of the object.
     *
     * @return array
     */
    public function __serialize()
    {}

    /**
     * Marshal the object from its serialized data.
     *
     * @param  array  $data
     * @return void
     */
    public function __unserialize(array $data)
    {}

    /**
     * Dynamically get the original message.
     *
     * @param  string  $key
     * @return mixed
     *
     * @throws \Exception
     */
    public function __get($key)
    {}
}
