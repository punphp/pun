<?php

namespace Illuminate\Broadcasting;

use Illuminate\Contracts\Broadcasting\HasBroadcastChannel;

class PrivateChannel extends Channel
{
    /**
     * Create a new channel instance.
     *
     * @param  \Illuminate\Contracts\Broadcasting\HasBroadcastChannel|string  $name
     */
    public function __construct($name)
    {}
}
