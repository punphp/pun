<?php

namespace Illuminate\Cache\Events;

class WritingManyKeys extends CacheEvent
{
    /**
     * The keys that are being written.
     *
     * @var mixed
     */
    public $keys;

    /**
     * The value that is being written.
     *
     * @var mixed
     */
    public $values;

    /**
     * The number of seconds the keys should be valid.
     *
     * @var int|null
     */
    public $seconds;

    /**
     * Create a new event instance.
     *
     * @param  string|null  $storeName
     * @param  array  $keys
     * @param  array  $values
     * @param  int|null  $seconds
     * @param  array  $tags
     */
    public function __construct($storeName, $keys, $values, $seconds = null, $tags = [])
    {}
}
