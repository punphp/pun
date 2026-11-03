<?php

namespace Illuminate\Translation;

use Stringable;

class PotentiallyTranslatedString implements Stringable
{
    /**
     * The string that may be translated.
     *
     * @var string
     */
    protected $string;

    /**
     * The translated string.
     *
     * @var string|null
     */
    protected $translation;

    /**
     * The validator that may perform the translation.
     *
     * @var \Illuminate\Contracts\Translation\Translator
     */
    protected $translator;

    /**
     * Create a new potentially translated string.
     *
     * @param  string  $string
     * @param  \Illuminate\Contracts\Translation\Translator  $translator
     */
    public function __construct($string, $translator)
    {}

    /**
     * Translate the string.
     *
     * @param  array  $replace
     * @param  string|null  $locale
     * @return $this
     */
    public function translate($replace = [], $locale = null)
    {}

    /**
     * Translates the string based on a count.
     *
     * @param  \Countable|int|float|array  $number
     * @param  array  $replace
     * @param  string|null  $locale
     * @return $this
     */
    public function translateChoice($number, array $replace = [], $locale = null)
    {}

    /**
     * Get the original string.
     *
     * @return string
     */
    public function original()
    {}

    /**
     * Get the potentially translated string.
     *
     * @return string
     */
    public function __toString()
    {}

    /**
     * Get the potentially translated string.
     *
     * @return string
     */
    public function toString()
    {}
}
