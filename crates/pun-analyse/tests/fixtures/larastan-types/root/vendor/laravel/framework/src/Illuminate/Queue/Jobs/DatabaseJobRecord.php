<?php

namespace Illuminate\Queue\Jobs;

use Illuminate\Support\InteractsWithTime;

class DatabaseJobRecord
{
    use InteractsWithTime;

    /**
     * The underlying job record.
     *
     * @var \stdClass
     */
    protected $record;

    /**
     * Create a new job record instance.
     *
     * @param  \stdClass  $record
     */
    public function __construct($record)
    {}

    /**
     * Increment the number of times the job has been attempted.
     *
     * @return int
     */
    public function increment()
    {}

    /**
     * Update the "reserved at" timestamp of the job.
     *
     * @return int
     */
    public function touch()
    {}

    /**
     * Dynamically access the underlying job information.
     *
     * @param  string  $key
     * @return mixed
     */
    public function __get($key)
    {}
}
