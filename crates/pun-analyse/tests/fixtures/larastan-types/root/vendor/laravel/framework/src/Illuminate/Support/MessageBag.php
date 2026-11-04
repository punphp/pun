<?php

namespace Illuminate\Support;

use Illuminate\Contracts\Support\Arrayable;
use Illuminate\Contracts\Support\Jsonable;
use Illuminate\Contracts\Support\MessageBag as MessageBagContract;
use Illuminate\Contracts\Support\MessageProvider;
use JsonSerializable;
use Stringable;

class MessageBag implements Jsonable, JsonSerializable, MessageBagContract, MessageProvider, Stringable
{
    /**
     * All of the registered messages.
     *
     * @var array<string, array<string>>
     */
    protected $messages = [];

    /**
     * Default format for message output.
     *
     * @var string
     */
    protected $format = ':message';

    /**
     * Create a new message bag instance.
     *
     * @param  array<string, Arrayable|string|array<string>>  $messages
     */
    public function __construct(array $messages = [])
    {}

    /**
     * Get the keys present in the message bag.
     *
     * @return array<string>
     */
    public function keys()
    {}

    /**
     * Add a message to the message bag.
     *
     * @param  string  $key
     * @param  string  $message
     * @return $this
     */
    public function add($key, $message)
    {}

    /**
     * Add a message to the message bag if the given conditional is "true".
     *
     * @param  bool  $boolean
     * @param  string  $key
     * @param  string  $message
     * @return $this
     */
    public function addIf($boolean, $key, $message)
    {}

    /**
     * Determine if a key and message combination already exists.
     *
     * @param  string  $key
     * @param  string  $message
     * @return bool
     */
    protected function isUnique($key, $message)
    {}

    /**
     * Merge a new array of messages into the message bag.
     *
     * @param  \Illuminate\Contracts\Support\MessageProvider|array<string, array<string>>  $messages
     * @return $this
     */
    public function merge($messages)
    {}

    /**
     * Determine if messages exist for all of the given keys.
     *
     * @param  array<string>|string|null  $key
     * @return bool
     */
    public function has($key)
    { func_get_args(); }

    /**
     * Determine if messages exist for any of the given keys.
     *
     * @param  array<string>|string|null  $keys
     * @return bool
     */
    public function hasAny($keys = [])
    { func_get_args(); }

    /**
     * Determine if messages don't exist for all of the given keys.
     *
     * @param  array<string>|string|null  $key
     * @return bool
     */
    public function missing($key)
    { func_get_args(); }

    /**
     * Get the first message from the message bag for a given key.
     *
     * @param  string|null  $key
     * @param  string|null  $format
     * @return string
     */
    public function first($key = null, $format = null)
    {}

    /**
     * Get all of the messages from the message bag for a given key.
     *
     * @param  string  $key
     * @param  string|null  $format
     * @return array<string>|array<string, array<string>>
     */
    public function get($key, $format = null)
    {}

    /**
     * Get the messages for a wildcard key.
     *
     * @param  string  $key
     * @param  string|null  $format
     * @return array<string, array<string>>
     */
    protected function getMessagesForWildcardKey($key, $format)
    {}

    /**
     * Get all of the messages for every key in the message bag.
     *
     * @param  string|null  $format
     * @return array<string>
     */
    public function all($format = null)
    {}

    /**
     * Get all of the unique messages for every key in the message bag.
     *
     * @param  string|null  $format
     * @return array<string>
     */
    public function unique($format = null)
    {}

    /**
     * Remove a message from the message bag.
     *
     * @param  string  $key
     * @return $this
     */
    public function forget($key)
    {}

    /**
     * Format an array of messages.
     *
     * @param  array<string>  $messages
     * @param  string  $format
     * @param  string  $messageKey
     * @return array<string>
     */
    protected function transform($messages, $format, $messageKey)
    {}

    /**
     * Get the appropriate format based on the given format.
     *
     * @param  string|null  $format
     * @return string
     */
    protected function checkFormat($format)
    {}

    /**
     * Get the raw messages in the message bag.
     *
     * @return array<string, array<string>>
     */
    public function messages()
    {}

    /**
     * Get the raw messages in the message bag.
     *
     * @return array<string, array<string>>
     */
    public function getMessages()
    {}

    /**
     * Get the messages for the instance.
     *
     * @return \Illuminate\Support\MessageBag
     */
    public function getMessageBag()
    {}

    /**
     * Get the default message format.
     *
     * @return string
     */
    public function getFormat()
    {}

    /**
     * Set the default message format.
     *
     * @param  string  $format
     * @return \Illuminate\Support\MessageBag
     */
    public function setFormat($format = ':message')
    {}

    /**
     * Determine if the message bag has any messages.
     *
     * @return bool
     */
    public function isEmpty()
    {}

    /**
     * Determine if the message bag has any messages.
     *
     * @return bool
     */
    public function isNotEmpty()
    {}

    /**
     * Determine if the message bag has any messages.
     *
     * @return bool
     */
    public function any()
    {}

    /**
     * Get the number of messages in the message bag.
     *
     * @return int
     */
    public function count(): int
    {}

    /**
     * Get the instance as an array.
     *
     * @return array<string, array<string>>
     */
    public function toArray()
    {}

    /**
     * Convert the object into something JSON serializable.
     *
     * @return array<string, array<string>>
     */
    public function jsonSerialize(): array
    {}

    /**
     * Convert the object to its JSON representation.
     *
     * @param  int  $options
     * @return string
     */
    public function toJson($options = 0)
    {}

    /**
     * Convert the object to pretty print formatted JSON.
     *
     * @param  int  $options
     * @return string
     */
    public function toPrettyJson(int $options = 0)
    {}

    /**
     * Convert the message bag to its string representation.
     *
     * @return string
     */
    public function __toString()
    {}
}
