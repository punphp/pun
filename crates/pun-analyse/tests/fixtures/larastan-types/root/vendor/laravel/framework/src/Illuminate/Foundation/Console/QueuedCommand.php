<?php

namespace Illuminate\Foundation\Console;

use Illuminate\Bus\Queueable;
use Illuminate\Contracts\Console\Kernel as KernelContract;
use Illuminate\Contracts\Queue\ShouldQueue;
use Illuminate\Foundation\Bus\Dispatchable;

class QueuedCommand implements ShouldQueue
{
    use Dispatchable, Queueable;

    /**
     * The data to pass to the Artisan command.
     *
     * @var array
     */
    protected $data;

    /**
     * Create a new job instance.
     *
     * @param  array  $data
     */
    public function __construct($data)
    {}

    /**
     * Handle the job.
     *
     * @param  \Illuminate\Contracts\Console\Kernel  $kernel
     * @return void
     */
    public function handle(KernelContract $kernel)
    {}

    /**
     * Get the display name for the queued job.
     *
     * @return string
     */
    public function displayName()
    {}
}
