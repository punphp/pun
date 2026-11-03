<?php

namespace Illuminate\Cache;

class LuaScripts
{
    /**
     * Get the Lua script that sets a key only when it does not yet exist.
     *
     * KEYS[1] - The name of the key
     * ARGV[1] - The value of the key
     * ARGV[2] - The number of seconds the key should be valid
     *
     * @return string
     */
    public static function add()
    {}

    /**
     * Get the Lua script to atomically refresh a lock's expiration.
     *
     * KEYS[1] - The name of the lock
     * ARGV[1] - The owner key of the lock instance trying to refresh it
     * ARGV[2] - The number of seconds the lock should be valid
     *
     * @return string
     */
    public static function refreshLock()
    {}

    /**
     * Get the Lua script to atomically release a lock.
     *
     * KEYS[1] - The name of the lock
     * ARGV[1] - The owner key of the lock instance trying to release it
     *
     * @return string
     */
    public static function releaseLock()
    {}
}
