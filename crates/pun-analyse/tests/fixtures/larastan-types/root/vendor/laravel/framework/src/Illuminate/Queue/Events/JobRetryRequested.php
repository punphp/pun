<?php

namespace Illuminate\Queue\Events;

class JobRetryRequested
{
    /**
     * The decoded job payload.
     *
     * @var array|null
     */
    protected $payload = null;

    /**
     * Create a new event instance.
     *
     * @param  \stdClass  $job  The job instance.
     */
    public function __construct(
        public $job,
    ) {}

    /**
     * The job payload.
     *
     * @return array
     */
    public function payload()
    {}
}
