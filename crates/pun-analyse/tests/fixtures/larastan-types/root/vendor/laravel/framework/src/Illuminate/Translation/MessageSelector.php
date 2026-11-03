<?php

namespace Illuminate\Translation;

use Illuminate\Support\Collection;

class MessageSelector
{
    /**
     * Select a proper translation string based on the given number.
     *
     * @param  string  $line
     * @param  int|float  $number
     * @param  string  $locale
     * @return mixed
     */
    public function choose($line, $number, $locale)
    {}

    /**
     * Extract a translation string using inline conditions.
     *
     * @param  array  $segments
     * @param  int|float  $number
     * @return mixed
     */
    private function extract($segments, $number)
    {}

    /**
     * Get the translation string if the condition matches.
     *
     * @param  string  $part
     * @param  int  $number
     * @return mixed
     */
    private function extractFromString($part, $number)
    {}

    /**
     * Strip the inline conditions from each segment, just leaving the text.
     *
     * @param  array  $segments
     * @return array
     */
    private function stripConditions($segments)
    {}

    /**
     * Get the index to use for pluralization.
     *
     * The plural rules are derived from code of the Zend Framework (2010-09-25), which
     * is subject to the new BSD license (https://framework.zend.com/license)
     * Copyright (c) 2005-2010 - Zend Technologies USA Inc. (http://www.zend.com)
     *
     * @param  string  $locale
     * @param  int|float  $number
     * @return int
     */
    public function getPluralIndex($locale, $number)
    {}
}
