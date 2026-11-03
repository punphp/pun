<?php

namespace Illuminate\Http\Testing;

use Illuminate\Support\Arr;
use Symfony\Component\Mime\MimeTypes;

class MimeType
{
    /**
     * The MIME types instance.
     *
     * @var \Symfony\Component\Mime\MimeTypes|null
     */
    private static $mime;

    /**
     * Get the MIME types instance.
     *
     * @return \Symfony\Component\Mime\MimeTypesInterface
     */
    public static function getMimeTypes()
    {}

    /**
     * Get the MIME type for a file based on the file's extension.
     *
     * @param  string  $filename
     * @return string
     */
    public static function from($filename)
    {}

    /**
     * Get the MIME type for a given extension or return all MIME types.
     *
     * @param  string  $extension
     * @return string
     */
    public static function get($extension)
    {}

    /**
     * Search for the extension of a given MIME type.
     *
     * @param  string  $mimeType
     * @return string|null
     */
    public static function search($mimeType)
    {}
}
