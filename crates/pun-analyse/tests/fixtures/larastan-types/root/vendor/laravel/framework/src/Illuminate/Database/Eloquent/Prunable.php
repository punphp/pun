<?php

namespace Illuminate\Database\Eloquent;

use Illuminate\Contracts\Debug\ExceptionHandler;
use Illuminate\Database\Events\ModelsPruned;
use LogicException;
use Throwable;

trait Prunable
{
    /**
     * Prune all prunable models in the database.
     *
     * @param  int  $chunkSize
     * @return int
     *
     * @throws \Throwable
     */
    public function pruneAll(int $chunkSize = 1000)
    {}

    /**
     * Get the prunable model query.
     *
     * @return \Illuminate\Database\Eloquent\Builder<static>
     *
     * @throws \LogicException
     */
    public function prunable()
    {}

    /**
     * Prune the model in the database.
     *
     * @return bool|null
     */
    public function prune()
    {}

    /**
     * Prepare the model for pruning.
     *
     * @return void
     */
    protected function pruning()
    {}
}
