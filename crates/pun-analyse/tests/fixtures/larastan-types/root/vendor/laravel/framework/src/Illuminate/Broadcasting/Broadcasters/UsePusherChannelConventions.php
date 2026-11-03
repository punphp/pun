<?php

namespace Illuminate\Broadcasting\Broadcasters;

use Illuminate\Support\Str;

trait UsePusherChannelConventions
{
    /**
     * Return true if the channel is protected by authentication.
     *
     * @param  string  $channel
     * @return bool
     */
    public function isGuardedChannel($channel)
    {}

    /**
     * Remove prefix from channel name.
     *
     * @param  string  $channel
     * @return string
     */
    public function normalizeChannelName($channel)
    {}
}
