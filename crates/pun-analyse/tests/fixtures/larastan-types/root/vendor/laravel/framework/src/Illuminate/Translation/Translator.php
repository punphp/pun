<?php

namespace Illuminate\Translation;

use Closure;
use Illuminate\Contracts\Translation\Loader;
use Illuminate\Contracts\Translation\Translator as TranslatorContract;
use Illuminate\Support\Arr;
use Illuminate\Support\NamespacedItemResolver;
use Illuminate\Support\Str;
use Illuminate\Support\Traits\Macroable;
use Illuminate\Support\Traits\ReflectsClosures;
use InvalidArgumentException;

use function Illuminate\Support\enum_value;

class Translator extends NamespacedItemResolver implements TranslatorContract
{
    use Macroable, ReflectsClosures;

    /**
     * The loader implementation.
     *
     * @var \Illuminate\Contracts\Translation\Loader
     */
    protected $loader;

    /**
     * The default locale being used by the translator.
     *
     * @var string
     */
    protected $locale;

    /**
     * The fallback locale used by the translator.
     *
     * @var string
     */
    protected $fallback;

    /**
     * The array of loaded translation groups.
     *
     * @var array
     */
    protected $loaded = [];

    /**
     * The message selector.
     *
     * @var \Illuminate\Translation\MessageSelector|null
     */
    protected $selector;

    /**
     * The callable that should be invoked to determine applicable locales.
     *
     * @var callable
     */
    protected $determineLocalesUsing;

    /**
     * The custom rendering callbacks for stringable objects.
     *
     * @var array
     */
    protected $stringableHandlers = [];

    /**
     * The callback that is responsible for handling missing translation keys.
     *
     * @var callable|null
     */
    protected $missingTranslationKeyCallback;

    /**
     * Indicates whether missing translation keys should be handled.
     *
     * @var bool
     */
    protected $handleMissingTranslationKeys = true;

    /**
     * Create a new translator instance.
     *
     * @param  \Illuminate\Contracts\Translation\Loader  $loader
     * @param  string  $locale
     */
    public function __construct(Loader $loader, $locale)
    {}

    /**
     * Determine if a translation exists for a given locale.
     *
     * @param  string  $key
     * @param  string|null  $locale
     * @return bool
     */
    public function hasForLocale($key, $locale = null)
    {}

    /**
     * Determine if a translation exists.
     *
     * @param  string  $key
     * @param  string|null  $locale
     * @param  bool  $fallback
     * @return bool
     */
    public function has($key, $locale = null, $fallback = true)
    {}

    /**
     * Get the translation for the given key.
     *
     * @param  string  $key
     * @param  array  $replace
     * @param  string|null  $locale
     * @param  bool  $fallback
     * @return string|array
     */
    public function get($key, array $replace = [], $locale = null, $fallback = true)
    {}

    /**
     * Get the specified string translation value.
     *
     * @throws \InvalidArgumentException
     */
    public function string(string $key, array $replace = [], ?string $locale = null, bool $fallback = true): string
    {}

    /**
     * Get the specified array translation value.
     *
     * @return array<array-key, mixed>
     *
     * @throws \InvalidArgumentException
     */
    public function array(string $key, array $replace = [], ?string $locale = null, bool $fallback = true): array
    {}

    /**
     * Get a translation according to an integer value.
     *
     * @param  string  $key
     * @param  \Countable|int|float|array  $number
     * @param  array  $replace
     * @param  string|null  $locale
     * @return string
     */
    public function choice($key, $number, array $replace = [], $locale = null)
    {}

    /**
     * Get the proper locale for a choice operation.
     *
     * @param  string  $key
     * @param  string|null  $locale
     * @return string
     */
    protected function localeForChoice($key, $locale)
    {}

    /**
     * Retrieve a language line out the loaded array.
     *
     * @param  string  $namespace
     * @param  string  $group
     * @param  string  $locale
     * @param  string  $item
     * @param  array  $replace
     * @return string|array|null
     */
    protected function getLine($namespace, $group, $locale, $item, array $replace)
    {}

    /**
     * Make the place-holder replacements on a line.
     *
     * @param  string  $line
     * @param  array  $replace
     * @return string
     */
    protected function makeReplacements($line, array $replace)
    {}

    /**
     * Add translation lines to the given locale.
     *
     * @param  array  $lines
     * @param  string  $locale
     * @param  string  $namespace
     * @return void
     */
    public function addLines(array $lines, $locale, $namespace = '*')
    {}

    /**
     * Load the specified language group.
     *
     * @param  string  $namespace
     * @param  string  $group
     * @param  string  $locale
     * @return void
     */
    public function load($namespace, $group, $locale)
    {}

    /**
     * Determine if the given group has been loaded.
     *
     * @param  string  $namespace
     * @param  string  $group
     * @param  string  $locale
     * @return bool
     */
    protected function isLoaded($namespace, $group, $locale)
    {}

    /**
     * Handle a missing translation key.
     *
     * @param  string  $key
     * @param  array  $replace
     * @param  string|null  $locale
     * @param  bool  $fallback
     * @return string
     */
    protected function handleMissingTranslationKey($key, $replace, $locale, $fallback)
    {}

    /**
     * Register a callback that is responsible for handling missing translation keys.
     *
     * @param  callable|null  $callback
     * @return $this
     */
    public function handleMissingKeysUsing(?callable $callback)
    {}

    /**
     * Add a new namespace to the loader.
     *
     * @param  string  $namespace
     * @param  string  $hint
     * @return void
     */
    public function addNamespace($namespace, $hint)
    {}

    /**
     * Add a new path to the loader.
     *
     * @param  string  $path
     * @return void
     */
    public function addPath($path)
    {}

    /**
     * Add a new JSON path to the loader.
     *
     * @param  string  $path
     * @return void
     */
    public function addJsonPath($path)
    {}

    /**
     * Parse a key into namespace, group, and item.
     *
     * @param  string  $key
     * @return array
     */
    public function parseKey($key)
    {}

    /**
     * Get the array of locales to be checked.
     *
     * @param  string|null  $locale
     * @return array
     */
    protected function localeArray($locale)
    {}

    /**
     * Specify a callback that should be invoked to determined the applicable locale array.
     *
     * @param  callable  $callback
     * @return void
     */
    public function determineLocalesUsing($callback)
    {}

    /**
     * Get the message selector instance.
     *
     * @return \Illuminate\Translation\MessageSelector
     */
    public function getSelector()
    {}

    /**
     * Set the message selector instance.
     *
     * @param  \Illuminate\Translation\MessageSelector  $selector
     * @return void
     */
    public function setSelector(MessageSelector $selector)
    {}

    /**
     * Get the language line loader implementation.
     *
     * @return \Illuminate\Contracts\Translation\Loader
     */
    public function getLoader()
    {}

    /**
     * Get the default locale being used.
     *
     * @return string
     */
    public function locale()
    {}

    /**
     * Get the default locale being used.
     *
     * @return string
     */
    public function getLocale()
    {}

    /**
     * Set the default locale.
     *
     * @param  string  $locale
     * @return void
     *
     * @throws \InvalidArgumentException
     */
    public function setLocale($locale)
    {}

    /**
     * Get the fallback locale being used.
     *
     * @return string
     */
    public function getFallback()
    {}

    /**
     * Set the fallback locale being used.
     *
     * @param  string  $fallback
     * @return void
     */
    public function setFallback($fallback)
    {}

    /**
     * Set the loaded translation groups.
     *
     * @param  array  $loaded
     * @return void
     */
    public function setLoaded(array $loaded)
    {}

    /**
     * Add a handler to be executed in order to format a given class to a string during translation replacements.
     *
     * @param  callable|string  $class
     * @param  callable|null  $handler
     * @return void
     */
    public function stringable($class, $handler = null)
    {}
}
