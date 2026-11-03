<?php

namespace Illuminate\Http;

use Illuminate\Container\Container;
use Illuminate\Contracts\Filesystem\Factory as FilesystemFactory;
use Illuminate\Contracts\Filesystem\FileNotFoundException;
use Illuminate\Http\Testing\FileFactory;
use Illuminate\Support\Arr;
use Illuminate\Support\Traits\Macroable;
use Symfony\Component\HttpFoundation\File\UploadedFile as SymfonyUploadedFile;

class UploadedFile extends SymfonyUploadedFile
{
    use FileHelpers, Macroable;

    /**
     * Begin creating a new file fake.
     *
     * @return \Illuminate\Http\Testing\FileFactory
     */
    public static function fake()
    {}

    /**
     * Store the uploaded file on a filesystem disk.
     *
     * @param  string  $path
     * @param  array|string  $options
     * @return string|false
     */
    public function store($path = '', $options = [])
    {}

    /**
     * Store the uploaded file on a filesystem disk with public visibility.
     *
     * @param  string  $path
     * @param  array|string  $options
     * @return string|false
     */
    public function storePublicly($path = '', $options = [])
    {}

    /**
     * Store the uploaded file on a filesystem disk with public visibility.
     *
     * @param  string  $path
     * @param  array|string|null  $name
     * @param  array|string  $options
     * @return string|false
     */
    public function storePubliclyAs($path, $name = null, $options = [])
    {}

    /**
     * Store the uploaded file on a filesystem disk.
     *
     * @param  string  $path
     * @param  array|string|null  $name
     * @param  array|string  $options
     * @return string|false
     */
    public function storeAs($path, $name = null, $options = [])
    {}

    /**
     * Get the contents of the uploaded file.
     *
     * @return false|string
     *
     * @throws \Illuminate\Contracts\Filesystem\FileNotFoundException
     */
    public function get()
    {}

    /**
     * Get the file's extension supplied by the client.
     *
     * @return string
     */
    public function clientExtension()
    {}

    /**
     * Create a new file instance from a base instance.
     *
     * @param  \Symfony\Component\HttpFoundation\File\UploadedFile  $file
     * @param  bool  $test
     * @return static
     */
    public static function createFromBase(SymfonyUploadedFile $file, $test = false)
    {}

    /**
     * Parse and format the given options.
     *
     * @param  array|string  $options
     * @return array
     */
    protected function parseOptions($options)
    {}
}
