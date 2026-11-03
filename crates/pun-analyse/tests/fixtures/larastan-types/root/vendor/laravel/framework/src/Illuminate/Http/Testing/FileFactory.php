<?php

namespace Illuminate\Http\Testing;

use LogicException;

class FileFactory
{
    /**
     * Create a new fake file.
     *
     * @param  string  $name
     * @param  string|int  $kilobytes
     * @param  string|null  $mimeType
     * @return \Illuminate\Http\Testing\File
     */
    public function create($name, $kilobytes = 0, $mimeType = null)
    {}

    /**
     * Create a new fake file with content.
     *
     * @param  string  $name
     * @param  string  $content
     * @return \Illuminate\Http\Testing\File
     */
    public function createWithContent($name, $content)
    {}

    /**
     * Create a new fake image.
     *
     * @param  string  $name
     * @param  int  $width
     * @param  int  $height
     * @return \Illuminate\Http\Testing\File
     *
     * @throws \LogicException
     */
    public function image($name, $width = 10, $height = 10)
    {}

    /**
     * Generate a dummy image of the given width and height.
     *
     * @param  int  $width
     * @param  int  $height
     * @param  string  $extension
     * @return resource
     *
     * @throws \LogicException
     */
    protected function generateImage($width, $height, $extension)
    {}
}
