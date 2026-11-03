<?php

namespace Illuminate\View\Concerns;

use Illuminate\Contracts\View\View;
use Illuminate\Support\Str;
use InvalidArgumentException;

trait ManagesLayouts
{
    /**
     * All of the finished, captured sections.
     *
     * @var array
     */
    protected $sections = [];

    /**
     * The stack of in-progress sections.
     *
     * @var array
     */
    protected $sectionStack = [];

    /**
     * The parent placeholder for the request.
     *
     * @var mixed
     */
    protected static $parentPlaceholder = [];

    /**
     * The parent placeholder salt for the request.
     *
     * @var string
     */
    protected static $parentPlaceholderSalt;

    /**
     * Start injecting content into a section.
     *
     * @param  string  $section
     * @param  string|null  $content
     * @return void
     */
    public function startSection($section, $content = null)
    {}

    /**
     * Inject inline content into a section.
     *
     * @param  string  $section
     * @param  string  $content
     * @return void
     */
    public function inject($section, $content)
    {}

    /**
     * Stop injecting content into a section and return its contents.
     *
     * @return string
     */
    public function yieldSection()
    {}

    /**
     * Stop injecting content into a section.
     *
     * @param  bool  $overwrite
     * @return string
     *
     * @throws \InvalidArgumentException
     */
    public function stopSection($overwrite = false)
    {}

    /**
     * Stop injecting content into a section and append it.
     *
     * @return string
     *
     * @throws \InvalidArgumentException
     */
    public function appendSection()
    {}

    /**
     * Append content to a given section.
     *
     * @param  string  $section
     * @param  string  $content
     * @return void
     */
    protected function extendSection($section, $content)
    {}

    /**
     * Get the string contents of a section.
     *
     * @param  string  $section
     * @param  string  $default
     * @return string
     */
    public function yieldContent($section, $default = '')
    {}

    /**
     * Get the parent placeholder for the current request.
     *
     * @param  string  $section
     * @return string
     */
    public static function parentPlaceholder($section = '')
    {}

    /**
     * Get the parent placeholder salt.
     *
     * @return string
     */
    protected static function parentPlaceholderSalt()
    {}

    /**
     * Check if section exists.
     *
     * @param  string  $name
     * @return bool
     */
    public function hasSection($name)
    {}

    /**
     * Check if section does not exist.
     *
     * @param  string  $name
     * @return bool
     */
    public function sectionMissing($name)
    {}

    /**
     * Get the contents of a section.
     *
     * @param  string  $name
     * @param  string|null  $default
     * @return mixed
     */
    public function getSection($name, $default = null)
    {}

    /**
     * Get the entire array of sections.
     *
     * @return array
     */
    public function getSections()
    {}

    /**
     * Flush all of the sections.
     *
     * @return void
     */
    public function flushSections()
    {}
}
