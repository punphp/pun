<?php

namespace Illuminate\Contracts\Queue;

use InvalidArgumentException;

class EntityNotFoundException extends InvalidArgumentException
{
    /**
     * Create a new exception instance.
     *
     * @param  string  $type
     * @param  mixed  $id
     */
    public function __construct($type, $id)
    {}
}
