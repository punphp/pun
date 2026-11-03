<?php

namespace Illuminate\Queue\Console;

use Illuminate\Console\Command;
use Illuminate\Queue\Failed\PrunableFailedJobProvider;
use Illuminate\Support\Carbon;
use Symfony\Component\Console\Attribute\AsCommand;

#[AsCommand(name: 'queue:prune-failed')]
class PruneFailedJobsCommand extends Command
{
    /**
     * The console command signature.
     *
     * @var string
     */
    protected $signature = 'queue:prune-failed
                {--hours=24 : The number of hours to retain failed jobs data}';

    /**
     * The console command description.
     *
     * @var string
     */
    protected $description = 'Prune stale entries from the failed jobs table';

    /**
     * Execute the console command.
     *
     * @return int|null
     */
    public function handle()
    {}
}
