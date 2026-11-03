<?php

namespace Illuminate\Database;

use Illuminate\Contracts\Database\LostConnectionDetector as LostConnectionDetectorContract;
use Illuminate\Support\Str;
use Throwable;

class LostConnectionDetector implements LostConnectionDetectorContract
{
    /**
     * Determine if the given exception was caused by a lost connection.
     *
     * @param  \Throwable  $e
     * @return bool
     */
    public function causedByLostConnection(Throwable $e): bool
    {}
}
