<?php

namespace Illuminate\Process;

use Illuminate\Contracts\Process\ProcessResult as ProcessResultContract;
use OutOfBoundsException;

class FakeProcessSequence
{
    /**
     * The fake process results and descriptions.
     *
     * @var array
     */
    protected $processes = [];

    /**
     * Indicates that invoking this sequence when it is empty should throw an exception.
     *
     * @var bool
     */
    protected $failWhenEmpty = true;

    /**
     * The response that should be returned when the sequence is empty.
     *
     * @var \Illuminate\Contracts\Process\ProcessResult|\Illuminate\Process\FakeProcessDescription|null
     */
    protected $emptyProcess;

    /**
     * Create a new fake process sequence instance.
     *
     * @param  array  $processes
     */
    public function __construct(array $processes = [])
    {}

    /**
     * Push a new process result or description onto the sequence.
     *
     * @param  \Illuminate\Contracts\Process\ProcessResult|\Illuminate\Process\FakeProcessDescription|array|string  $process
     * @return $this
     */
    public function push(ProcessResultContract|FakeProcessDescription|array|string $process)
    {}

    /**
     * Make the sequence return a default result when it is empty.
     *
     * @param  \Illuminate\Contracts\Process\ProcessResult|\Illuminate\Process\FakeProcessDescription|array|string  $process
     * @return $this
     */
    public function whenEmpty(ProcessResultContract|FakeProcessDescription|array|string $process)
    {}

    /**
     * Convert the given result into an actual process result or description.
     *
     * @param  \Illuminate\Contracts\Process\ProcessResult|\Illuminate\Process\FakeProcessDescription|array|string  $process
     * @return \Illuminate\Contracts\Process\ProcessResult|\Illuminate\Process\FakeProcessDescription
     */
    protected function toProcessResult(ProcessResultContract|FakeProcessDescription|array|string $process)
    {}

    /**
     * Make the sequence return a default result when it is empty.
     *
     * @return $this
     */
    public function dontFailWhenEmpty()
    {}

    /**
     * Indicate that this sequence has depleted all of its process results.
     *
     * @return bool
     */
    public function isEmpty()
    {}

    /**
     * Get the next process in the sequence.
     *
     * @return \Illuminate\Contracts\Process\ProcessResult|\Illuminate\Process\FakeProcessDescription
     *
     * @throws \OutOfBoundsException
     */
    public function __invoke()
    {}
}
