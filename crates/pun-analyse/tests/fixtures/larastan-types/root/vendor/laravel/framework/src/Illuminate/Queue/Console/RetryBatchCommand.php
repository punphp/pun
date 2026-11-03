<?php

namespace Illuminate\Queue\Console;

use Illuminate\Bus\BatchRepository;
use Illuminate\Console\Command;
use Illuminate\Contracts\Console\Isolatable;
use Symfony\Component\Console\Attribute\AsCommand;

#[AsCommand(name: 'queue:retry-batch')]
class RetryBatchCommand extends Command implements Isolatable
{
    /**
     * The console command signature.
     *
     * @var string
     */
    protected $signature = 'queue:retry-batch
                            {id?* : The ID of the batch whose failed jobs should be retried}';

    /**
     * The console command description.
     *
     * @var string
     */
    protected $description = 'Retry the failed jobs for a batch';

    /**
     * Execute the console command.
     *
     * @return int
     */
    public function handle()
    {}

    /**
     * Get the custom mutex name for an isolated command.
     *
     * @return string
     */
    public function isolatableId()
    {}

    /**
     * Get the batch IDs to be retried.
     *
     * @return array
     */
    protected function getBatchJobIds()
    {}
}
