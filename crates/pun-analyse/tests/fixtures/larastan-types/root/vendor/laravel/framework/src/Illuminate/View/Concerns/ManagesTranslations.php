<?php

namespace Illuminate\View\Concerns;

trait ManagesTranslations
{
    /**
     * The translation replacements for the translation being rendered.
     *
     * @var array
     */
    protected $translationReplacements = [];

    /**
     * Start a translation block.
     *
     * @param  array  $replacements
     * @return void
     */
    public function startTranslation($replacements = [])
    {}

    /**
     * Render the current translation.
     *
     * @return string
     */
    public function renderTranslation()
    {}
}
