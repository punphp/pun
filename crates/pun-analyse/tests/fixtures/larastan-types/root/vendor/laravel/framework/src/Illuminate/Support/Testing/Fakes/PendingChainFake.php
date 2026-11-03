<?php

namespace Illuminate\Support\Testing\Fakes;

use Closure;
use Illuminate\Foundation\Bus\PendingChain;
use Illuminate\Queue\CallQueuedClosure;

class PendingChainFake extends PendingChain
{
    /**
     * The fake bus instance.
     *
     * @var \Illuminate\Support\Testing\Fakes\BusFake
     */
    protected $bus;

    /**
     * Create a new pending chain instance.
     *
     * @param  \Illuminate\Support\Testing\Fakes\BusFake  $bus
     * @param  mixed  $job
     * @param  array  $chain
     */
    public function __construct(BusFake $bus, $job, $chain)
    {}

    /**
     * Dispatch the job with the given arguments.
     *
     * @return \Illuminate\Foundation\Bus\PendingDispatch
     */
    public function dispatch()
    {}
}
