<?php

namespace Illuminate\Cache\Events;

class CacheFlushing
{
    /**
     * The name of the cache store.
     *
     * @var string|null
     */
    public $storeName;

    /**
     * The tags that were assigned to the key.
     *
     * @var array
     */
    public $tags;

    /**
     * Create a new event instance.
     *
     * @param  string|null  $storeName
     * @param  array  $tags
     */
    public function __construct($storeName, array $tags = [])
    {}

    /**
     * Set the tags for the cache event.
     *
     * @param  array  $tags
     * @return $this
     */
    public function setTags($tags)
    {}
}
