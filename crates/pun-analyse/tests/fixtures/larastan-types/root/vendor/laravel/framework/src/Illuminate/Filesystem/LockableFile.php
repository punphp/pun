<?php

namespace Illuminate\Filesystem;

use Illuminate\Contracts\Filesystem\LockTimeoutException;

class LockableFile
{
    /**
     * The file resource.
     *
     * @var resource
     */
    protected $handle;

    /**
     * The file path.
     *
     * @var string
     */
    protected $path;

    /**
     * Indicates if the file is locked.
     *
     * @var bool
     */
    protected $isLocked = false;

    /**
     * Create a new File instance.
     *
     * @param  string  $path
     * @param  string  $mode
     */
    public function __construct($path, $mode)
    {}

    /**
     * Create the file's directory if necessary.
     *
     * @param  string  $path
     * @return void
     */
    protected function ensureDirectoryExists($path)
    {}

    /**
     * Create the file resource.
     *
     * @param  string  $path
     * @param  string  $mode
     * @return void
     *
     * @throws \Exception
     */
    protected function createResource($path, $mode)
    {}

    /**
     * Read the file contents.
     *
     * @param  int|null  $length
     * @return string
     */
    public function read($length = null)
    {}

    /**
     * Get the file size.
     *
     * @return int
     */
    public function size()
    {}

    /**
     * Write to the file.
     *
     * @param  string  $contents
     * @return $this
     */
    public function write($contents)
    {}

    /**
     * Truncate the file.
     *
     * @return $this
     */
    public function truncate()
    {}

    /**
     * Get a shared lock on the file.
     *
     * @param  bool  $block
     * @return $this
     *
     * @throws \Illuminate\Contracts\Filesystem\LockTimeoutException
     */
    public function getSharedLock($block = false)
    {}

    /**
     * Get an exclusive lock on the file.
     *
     * @param  bool  $block
     * @return $this
     *
     * @throws \Illuminate\Contracts\Filesystem\LockTimeoutException
     */
    public function getExclusiveLock($block = false)
    {}

    /**
     * Release the lock on the file.
     *
     * @return $this
     */
    public function releaseLock()
    {}

    /**
     * Close the file.
     *
     * @return bool
     */
    public function close()
    {}
}
