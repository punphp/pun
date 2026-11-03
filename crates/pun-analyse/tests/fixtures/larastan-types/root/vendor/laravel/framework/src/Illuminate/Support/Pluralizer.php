<?php

namespace Illuminate\Support;

use Doctrine\Inflector\InflectorFactory;

class Pluralizer
{
    /**
     * The cached inflector instance.
     *
     * @var \Doctrine\Inflector\Inflector|null
     */
    protected static $inflector;

    /**
     * The language that should be used by the inflector.
     *
     * @var string
     */
    protected static $language = 'english';

    /**
     * Uncountable non-nouns word forms.
     *
     * Contains words supported by Doctrine/Inflector/Rules/English/Uninflected.php
     *
     * @var string[]
     */
    public static $uncountable = [
        'recommended',
        'related',
    ];

    /**
     * Get the plural form of an English word.
     *
     * @param  string  $value
     * @param  int|array|\Countable  $count
     * @return string
     */
    public static function plural($value, $count = 2)
    {}

    /**
     * Get the singular form of an English word.
     *
     * @param  string  $value
     * @return string
     */
    public static function singular($value)
    {}

    /**
     * Determine if the given value is uncountable.
     *
     * @param  string  $value
     * @return bool
     */
    protected static function uncountable($value)
    {}

    /**
     * Attempt to match the case on two strings.
     *
     * @param  string  $value
     * @param  string  $comparison
     * @return string
     */
    protected static function matchCase($value, $comparison)
    {}

    /**
     * Get the inflector instance.
     *
     * @return \Doctrine\Inflector\Inflector
     */
    public static function inflector()
    {}

    /**
     * Specify the language that should be used by the inflector.
     *
     * @param  string  $language
     * @return void
     */
    public static function useLanguage(string $language)
    {}
}
