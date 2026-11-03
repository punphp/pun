<?php

namespace Illuminate\Http\Concerns;

use Illuminate\Database\Eloquent\Model;

trait InteractsWithFlashData
{
    /**
     * Retrieve an old input item.
     *
     * @param  string|null  $key
     * @param  \Illuminate\Database\Eloquent\Model|string|array|null  $default
     * @return string|array|null
     */
    public function old($key = null, $default = null)
    {}

    /**
     * Flash the input for the current request to the session.
     *
     * @return void
     */
    public function flash()
    {}

    /**
     * Flash only some of the input to the session.
     *
     * @param  mixed  $keys
     * @return void
     */
    public function flashOnly($keys)
    {}

    /**
     * Flash only some of the input to the session.
     *
     * @param  mixed  $keys
     * @return void
     */
    public function flashExcept($keys)
    {}

    /**
     * Flush all of the old input from the session.
     *
     * @return void
     */
    public function flush()
    {}
}
