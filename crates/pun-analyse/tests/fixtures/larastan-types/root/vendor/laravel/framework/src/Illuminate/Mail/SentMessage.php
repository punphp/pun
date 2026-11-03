<?php

namespace Illuminate\Mail;

use Illuminate\Support\Collection;
use Illuminate\Support\Traits\ForwardsCalls;
use Symfony\Component\Mailer\SentMessage as SymfonySentMessage;

/**
 * @mixin \Symfony\Component\Mailer\SentMessage
 */
class SentMessage
{
    use ForwardsCalls;

    /**
     * The Symfony SentMessage instance.
     *
     * @var \Symfony\Component\Mailer\SentMessage
     */
    protected $sentMessage;

    /**
     * Create a new SentMessage instance.
     *
     * @param  \Symfony\Component\Mailer\SentMessage  $sentMessage
     */
    public function __construct(SymfonySentMessage $sentMessage)
    {}

    /**
     * Get the underlying Symfony Email instance.
     *
     * @return \Symfony\Component\Mailer\SentMessage
     */
    public function getSymfonySentMessage()
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
}
