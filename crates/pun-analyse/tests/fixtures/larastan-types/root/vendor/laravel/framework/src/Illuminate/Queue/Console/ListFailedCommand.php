<?php

namespace Illuminate\Queue\Console;

use Illuminate\Console\Command;
use Illuminate\Support\Arr;
use Illuminate\Support\Collection;
use Symfony\Component\Console\Attribute\AsCommand;

#[AsCommand(name: 'queue:failed')]
class ListFailedCommand extends Command
{
    /**
     * The console command name.
     *
     * @var string
     */
    protected $signature = 'queue:failed
                            {--queue= : Only list the failed jobs for the specified queue}
                            {--json : Output the failed jobs as JSON}';

    /**
     * The console command description.
     *
     * @var string
     */
    protected $description = 'List all of the failed queue jobs';

    /**
     * The table headers for the command.
     *
     * @var string[]
     */
    protected $headers = ['ID', 'Connection', 'Queue', 'Class', 'Failed At'];

    /**
     * Execute the console command.
     *
     * @return void
     */
    public function handle()
    {}

    /**
     * Compile the failed jobs into a displayable format.
     *
     * @return array
     */
    protected function getFailedJobs()
    {}

    /**
     * Parse the failed job row.
     *
     * @param  array  $failed
     * @return array
     */
    protected function parseFailedJob(array $failed)
    {}

    /**
     * Extract the failed job name from payload.
     *
     * @param  string  $payload
     * @return string|null
     */
    private function extractJobName($payload)
    {}

    /**
     * Match the job name from the payload.
     *
     * @param  array  $payload
     * @return string|null
     */
    protected function matchJobName($payload)
    {}

    /**
     * Display the failed jobs in the console.
     *
     * @param  array  $jobs
     * @return void
     */
    protected function displayFailedJobs(array $jobs)
    {}

    /**
     * Display the failed jobs as JSON.
     *
     * @param  array  $jobs
     * @return void
     */
    protected function displayFailedJobsAsJson(array $jobs)
    {}
}
