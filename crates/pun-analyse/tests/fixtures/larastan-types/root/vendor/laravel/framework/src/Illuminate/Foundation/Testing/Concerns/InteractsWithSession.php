<?php

namespace Illuminate\Foundation\Testing\Concerns;

trait InteractsWithSession
{
    /**
     * Set the session to the given array.
     *
     * @param  array  $data
     * @return $this
     */
    public function withSession(array $data)
    {}

    /**
     * Set the session to the given array.
     *
     * @param  array  $data
     * @return $this
     */
    public function session(array $data)
    {}

    /**
     * Start the session for the application.
     *
     * @return $this
     */
    protected function startSession()
    {}

    /**
     * Flush all of the current session data.
     *
     * @return $this
     */
    public function flushSession()
    {}
}
