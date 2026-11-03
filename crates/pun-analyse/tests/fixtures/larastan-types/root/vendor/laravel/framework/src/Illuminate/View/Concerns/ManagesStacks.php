<?php

namespace Illuminate\View\Concerns;

use InvalidArgumentException;

trait ManagesStacks
{
    /**
     * All of the finished, captured push sections.
     *
     * @var array
     */
    protected $pushes = [];

    /**
     * All of the finished, captured prepend sections.
     *
     * @var array
     */
    protected $prepends = [];

    /**
     * The stack of in-progress push sections.
     *
     * @var array
     */
    protected $pushStack = [];

    /**
     * Start injecting content into a push section.
     *
     * @param  string  $section
     * @param  string  $content
     * @return void
     */
    public function startPush($section, $content = '')
    {}

    /**
     * Stop injecting content into a push section.
     *
     * @return string
     *
     * @throws \InvalidArgumentException
     */
    public function stopPush()
    {}

    /**
     * Append content to a given push section.
     *
     * @param  string  $section
     * @param  string  $content
     * @return void
     */
    protected function extendPush($section, $content)
    {}

    /**
     * Start prepending content into a push section.
     *
     * @param  string  $section
     * @param  string  $content
     * @return void
     */
    public function startPrepend($section, $content = '')
    {}

    /**
     * Stop prepending content into a push section.
     *
     * @return string
     *
     * @throws \InvalidArgumentException
     */
    public function stopPrepend()
    {}

    /**
     * Prepend content to a given stack.
     *
     * @param  string  $section
     * @param  string  $content
     * @return void
     */
    protected function extendPrepend($section, $content)
    {}

    /**
     * Get the string contents of a push section.
     *
     * @param  string  $section
     * @param  string  $default
     * @return string
     */
    public function yieldPushContent($section, $default = '')
    {}

    /**
     * Determine if the stack has any content in it.
     */
    public function isStackEmpty(string $section): bool
    {}

    /**
     * Flush all of the stacks.
     *
     * @return void
     */
    public function flushStacks()
    {}
}
