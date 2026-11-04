<?php

namespace Illuminate\Cache;

use Illuminate\Contracts\Cache\Store;

abstract class TaggableStore implements Store
{
    /**
     * Begin executing a new tags operation.
     *
     * @param  mixed  $names
     * @return \Illuminate\Cache\TaggedCache
     */
    public function tags($names)
    { func_get_args(); }
}
