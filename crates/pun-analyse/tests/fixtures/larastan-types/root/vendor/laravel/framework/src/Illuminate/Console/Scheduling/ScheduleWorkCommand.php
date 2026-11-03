<?php

namespace Illuminate\Console\Scheduling;

use Illuminate\Console\Application;
use Illuminate\Console\Command;
use Illuminate\Support\Carbon;
use Illuminate\Support\ProcessUtils;
use Symfony\Component\Console\Attribute\AsCommand;
use Symfony\Component\Console\Output\OutputInterface;
use Symfony\Component\Process\Process;

#[AsCommand(name: 'schedule:work')]
class ScheduleWorkCommand extends Command
{
    /**
     * The name and signature of the console command.
     *
     * @var string
     */
    protected $signature = 'schedule:work
        {--run-output-file= : The file to direct <info>schedule:run</info> output to}
        {--whisper : Do not output message indicating that no jobs were ready to run}';

    /**
     * The console command description.
     *
     * @var string
     */
    protected $description = 'Start the schedule worker';

    /**
     * The "schedule:run" executions that are currently running.
     *
     * @var \Symfony\Component\Process\Process[]
     */
    protected $executions = [];

    /**
     * Indicates if the schedule worker should exit.
     *
     * @var bool
     */
    protected $shouldQuit = false;

    /**
     * Execute the console command.
     *
     * @return int
     */
    public function handle()
    {}

    /**
     * Run the schedule worker loop until it is signalled to stop.
     *
     * @param  string  $command
     * @return int
     */
    protected function work($command)
    {}

    /**
     * Listen for the signals that should terminate the schedule worker.
     *
     * @return void
     */
    protected function listenForSignals()
    {}

    /**
     * Sleep for a short period before the next worker tick.
     *
     * @return void
     */
    protected function sleep()
    {}
}
