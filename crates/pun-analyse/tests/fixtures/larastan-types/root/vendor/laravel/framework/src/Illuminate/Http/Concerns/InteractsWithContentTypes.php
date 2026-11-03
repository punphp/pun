<?php

namespace Illuminate\Http\Concerns;

use Illuminate\Support\Str;

trait InteractsWithContentTypes
{
    /**
     * Determine if the request is sending JSON.
     *
     * @return bool
     */
    public function isJson()
    {}

    /**
     * Determine if the current request probably expects a JSON response.
     *
     * @return bool
     */
    public function expectsJson()
    {}

    /**
     * Determine if the current request is asking for JSON.
     *
     * @return bool
     */
    public function wantsJson()
    {}

    /**
     * Determine if the current request is asking for Markdown.
     *
     * @return bool
     */
    public function wantsMarkdown()
    {}

    /**
     * Determines whether the current requests accepts a given content type.
     *
     * @param  string|array  $contentTypes
     * @return bool
     */
    public function accepts($contentTypes)
    {}

    /**
     * Return the most suitable content type from the given array based on content negotiation.
     *
     * @param  string|array  $contentTypes
     * @return string|null
     */
    public function prefers($contentTypes)
    {}

    /**
     * Determine if the current request accepts any content type.
     *
     * @return bool
     */
    public function acceptsAnyContentType()
    {}

    /**
     * Determines whether a request accepts JSON.
     *
     * @return bool
     */
    public function acceptsJson()
    {}

    /**
     * Determines whether a request accepts Markdown.
     *
     * @return bool
     */
    public function acceptsMarkdown()
    {}

    /**
     * Determines whether a request accepts HTML.
     *
     * @return bool
     */
    public function acceptsHtml()
    {}

    /**
     * Determine if the given content types match.
     *
     * @param  string  $actual
     * @param  string  $type
     * @return bool
     */
    public static function matchesType($actual, $type)
    {}

    /**
     * Get the data format expected in the response.
     *
     * @param  string  $default
     * @return string
     */
    public function format($default = 'html')
    {}
}
