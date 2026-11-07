<?php

namespace Illuminate\Concurrency;

use Carbon\CarbonInterval;
use Closure;
use Exception;
use Illuminate\Console\Application;
use Illuminate\Contracts\Concurrency\Driver;
use Illuminate\Process\Factory as ProcessFactory;
use Illuminate\Process\Pool;
use Illuminate\Support\Arr;
use Illuminate\Support\Defer\DeferredCallback;
use Illuminate\Support\Facades\Context;
use Illuminate\Support\Str;
use Laravel\SerializableClosure\SerializableClosure;
use RuntimeException;

use function Illuminate\Support\defer;

class ProcessDriver implements Driver
{
    /**
     * Create a new process based concurrency driver.
     */
    public function __construct(protected ProcessFactory $processFactory)
    {}

    /**
     * Run the given tasks concurrently and return an array containing the results.
     *
     * @throws \Throwable
     */
    public function run(Closure|array $tasks, CarbonInterval|int|null $timeout = null): array
    {}

    /**
     * Extract the signed result envelope from the given process output.
     *
     * @throws \RuntimeException
     */
    protected function parseEnvelope(string $output, string $key): array
    {}

    /**
     * Start the given tasks in the background after the current task has finished.
     */
    public function defer(Closure|array $tasks): DeferredCallback
    {}
}
