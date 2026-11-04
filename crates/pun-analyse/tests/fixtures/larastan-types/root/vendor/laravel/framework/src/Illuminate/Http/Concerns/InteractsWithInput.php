<?php

namespace Illuminate\Http\Concerns;

use Illuminate\Http\UploadedFile;
use Illuminate\Image\Image;
use Illuminate\Support\Arr;
use Illuminate\Support\Fluent;
use Illuminate\Support\Traits\Dumpable;
use Illuminate\Support\Traits\InteractsWithData;
use SplFileInfo;
use Symfony\Component\HttpFoundation\InputBag;

trait InteractsWithInput
{
    use Dumpable, InteractsWithData;

    /**
     * Retrieve a server variable from the request.
     *
     * @param  string|null  $key
     * @param  string|array|null  $default
     * @return string|array|null
     */
    public function server($key = null, $default = null)
    {}

    /**
     * Determine if a header is set on the request.
     *
     * @param  string  $key
     * @return bool
     */
    public function hasHeader($key)
    {}

    /**
     * Retrieve a header from the request.
     *
     * @param  string|null  $key
     * @param  string|array|null  $default
     * @return string|array|null
     */
    public function header($key = null, $default = null)
    {}

    /**
     * Get the bearer token from the request headers.
     *
     * @return string|null
     */
    public function bearerToken()
    {}

    /**
     * Get the keys for all of the input and files.
     *
     * @return array
     */
    public function keys()
    {}

    /**
     * Get all of the input and files for the request.
     *
     * @param  mixed  $keys
     * @return array
     */
    public function all($keys = null)
    { func_get_args(); }

    /**
     * Retrieve an input item from the request.
     *
     * @param  string|null  $key
     * @param  mixed  $default
     * @return mixed
     */
    public function input($key = null, $default = null)
    {}

    /**
     * Retrieve input from the request as a Fluent object instance.
     *
     * @param  array|string|null  $key
     * @param  array  $default
     * @return \Illuminate\Support\Fluent
     */
    public function fluent($key = null, array $default = [])
    {}

    /**
     * Retrieve a query string item from the request.
     *
     * @param  string|null  $key
     * @param  string|array|null  $default
     * @return string|array|null
     */
    public function query($key = null, $default = null)
    {}

    /**
     * Retrieve a request payload item from the request.
     *
     * @param  string|null  $key
     * @param  string|array|null  $default
     * @return string|array|null
     */
    public function post($key = null, $default = null)
    {}

    /**
     * Determine if a cookie is set on the request.
     *
     * @param  string  $key
     * @return bool
     */
    public function hasCookie($key)
    {}

    /**
     * Retrieve a cookie from the request.
     *
     * @param  string|null  $key
     * @param  string|array|null  $default
     * @return string|array|null
     */
    public function cookie($key = null, $default = null)
    {}

    /**
     * Get an array of all of the files on the request.
     *
     * @return array<string, \Illuminate\Http\UploadedFile|\Illuminate\Http\UploadedFile[]>
     */
    public function allFiles()
    {}

    /**
     * Convert the given array of Symfony UploadedFiles to custom Laravel UploadedFiles.
     *
     * @param  array<string, \Symfony\Component\HttpFoundation\File\UploadedFile|\Symfony\Component\HttpFoundation\File\UploadedFile[]>  $files
     * @return array<string, \Illuminate\Http\UploadedFile|\Illuminate\Http\UploadedFile[]>
     */
    protected function convertUploadedFiles(array $files)
    {}

    /**
     * Determine if the uploaded data contains a file.
     *
     * @param  string  $key
     * @return bool
     */
    public function hasFile($key)
    {}

    /**
     * Check that the given file is a valid file instance.
     *
     * @param  mixed  $file
     * @return bool
     */
    protected function isValidFile($file)
    {}

    /**
     * Retrieve a file from the request.
     *
     * @param  string|null  $key
     * @param  mixed  $default
     * @return ($key is null ? array<string, \Illuminate\Http\UploadedFile|\Illuminate\Http\UploadedFile[]> : \Illuminate\Http\UploadedFile|\Illuminate\Http\UploadedFile[]|null)
     */
    public function file($key = null, $default = null)
    {}

    /**
     * Retrieve a file from the request as an image instance.
     */
    public function image(string $key): ?Image
    {}

    /**
     * Retrieve data from the instance.
     *
     * @param  string|null  $key
     * @param  mixed  $default
     * @return mixed
     */
    protected function data($key = null, $default = null)
    {}

    /**
     * Retrieve a parameter item from a given source.
     *
     * @param  string  $source
     * @param  string|null  $key
     * @param  string|array|null  $default
     * @return string|array|null
     */
    protected function retrieveItem($source, $key, $default)
    {}

    /**
     * Dump the items.
     *
     * @param  mixed  $keys
     * @return $this
     */
    public function dump($keys = [])
    { func_get_args(); }
}
