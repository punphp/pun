<?php

namespace Illuminate\Database;

use Illuminate\Contracts\Database\ConcurrencyErrorDetector as ConcurrencyErrorDetectorContract;
use Illuminate\Support\Str;
use PDOException;
use Throwable;

class ConcurrencyErrorDetector implements ConcurrencyErrorDetectorContract
{
    /**
     * Determine if the given exception was caused by a concurrency error such as a deadlock or serialization failure.
     *
     * @param  \Throwable  $e
     * @return bool
     */
    public function causedByConcurrencyError(Throwable $e): bool
    {}
}
