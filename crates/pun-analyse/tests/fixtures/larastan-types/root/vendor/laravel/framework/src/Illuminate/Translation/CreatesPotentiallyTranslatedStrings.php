<?php

namespace Illuminate\Translation;

trait CreatesPotentiallyTranslatedStrings
{
    /**
     * Create a pending potentially translated string.
     *
     * @param  string  $attribute
     * @param  string|null  $message
     * @return \Illuminate\Translation\PotentiallyTranslatedString
     */
    protected function pendingPotentiallyTranslatedString($attribute, $message)
    {}
}
