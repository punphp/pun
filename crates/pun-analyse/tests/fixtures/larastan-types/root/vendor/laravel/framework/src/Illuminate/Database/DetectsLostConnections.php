<?php

namespace Illuminate\Database;

use Illuminate\Container\Container;
use Illuminate\Contracts\Database\LostConnectionDetector as LostConnectionDetectorContract;
use Throwable;

trait DetectsLostConnections
{
    /**
     * Determine if the given exception was caused by a lost connection.
     *
     * @param  \Throwable  $e
     * @return bool
     */
    protected function causedByLostConnection(Throwable $e)
    {}
}
