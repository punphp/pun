<?php

namespace Illuminate\Http;

use Illuminate\Support\Str;

trait FileHelpers
{
    /**
     * The cache copy of the file's hash name.
     *
     * @var string|null
     */
    protected $hashName = null;

    /**
     * Get the fully-qualified path to the file.
     *
     * @return string
     */
    public function path()
    {}

    /**
     * Get the file's extension.
     *
     * @return string|null
     */
    public function extension()
    {}

    /**
     * Get a filename for the file.
     *
     * @param  string|null  $path
     * @return string
     */
    public function hashName($path = null)
    {}

    /**
     * Get the dimensions of the image (if applicable).
     *
     * @return array|null
     */
    public function dimensions()
    {}
}
