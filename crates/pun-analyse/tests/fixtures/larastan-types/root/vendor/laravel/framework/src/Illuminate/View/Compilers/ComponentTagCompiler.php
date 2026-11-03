<?php

namespace Illuminate\View\Compilers;

use Illuminate\Container\Container;
use Illuminate\Contracts\Foundation\Application;
use Illuminate\Contracts\View\Factory;
use Illuminate\Filesystem\Filesystem;
use Illuminate\Support\Collection;
use Illuminate\Support\Str;
use Illuminate\View\AnonymousComponent;
use Illuminate\View\DynamicComponent;
use Illuminate\View\ViewFinderInterface;
use InvalidArgumentException;
use ReflectionClass;

/**
 * @author Spatie bvba <info@spatie.be>
 * @author Taylor Otwell <taylor@laravel.com>
 */
class ComponentTagCompiler
{
    /**
     * The Blade compiler instance.
     *
     * @var \Illuminate\View\Compilers\BladeCompiler
     */
    protected $blade;

    /**
     * The component class aliases.
     *
     * @var array
     */
    protected $aliases = [];

    /**
     * The component class namespaces.
     *
     * @var array
     */
    protected $namespaces = [];

    /**
     * The "bind:" attributes that have been compiled for the current component.
     *
     * @var array
     */
    protected $boundAttributes = [];

    /**
     * Create a new component tag compiler.
     *
     * @param  array  $aliases
     * @param  array  $namespaces
     * @param  \Illuminate\View\Compilers\BladeCompiler|null  $blade
     */
    public function __construct(array $aliases = [], array $namespaces = [], ?BladeCompiler $blade = null)
    {}

    /**
     * Compile the component and slot tags within the given string.
     *
     * @param  string  $value
     * @return string
     */
    public function compile(string $value)
    {}

    /**
     * Compile the tags within the given string.
     *
     * @param  string  $value
     * @return string
     *
     * @throws \InvalidArgumentException
     */
    public function compileTags(string $value)
    {}

    /**
     * Compile the opening tags within the given string.
     *
     * @param  string  $value
     * @return string
     *
     * @throws \InvalidArgumentException
     */
    protected function compileOpeningTags(string $value)
    {}

    /**
     * Compile the self-closing tags within the given string.
     *
     * @param  string  $value
     * @return string
     *
     * @throws \InvalidArgumentException
     */
    protected function compileSelfClosingTags(string $value)
    {}

    /**
     * Compile the Blade component string for the given component and attributes.
     *
     * @param  string  $component
     * @param  array  $attributes
     * @return string
     *
     * @throws \InvalidArgumentException
     */
    protected function componentString(string $component, array $attributes)
    {}

    /**
     * Get the component class for a given component alias.
     *
     * @param  string  $component
     * @return string
     *
     * @throws \InvalidArgumentException
     */
    public function componentClass(string $component)
    {}

    /**
     * Attempt to find an anonymous component using the registered anonymous component paths.
     *
     * @param  \Illuminate\Contracts\View\Factory  $viewFactory
     * @param  string  $component
     * @return string|null
     */
    protected function guessAnonymousComponentUsingPaths(Factory $viewFactory, string $component)
    {}

    /**
     * Attempt to find an anonymous component using the registered anonymous component namespaces.
     *
     * @param  \Illuminate\Contracts\View\Factory  $viewFactory
     * @param  string  $component
     * @return string|null
     */
    protected function guessAnonymousComponentUsingNamespaces(Factory $viewFactory, string $component)
    {}

    /**
     * Find the class for the given component using the registered namespaces.
     *
     * @param  string  $component
     * @return string|null
     */
    public function findClassByComponent(string $component)
    {}

    /**
     * Guess the class name for the given component.
     *
     * @param  string  $component
     * @return string
     */
    public function guessClassName(string $component)
    {}

    /**
     * Format the class name for the given component.
     *
     * @param  string  $component
     * @return string
     */
    public function formatClassName(string $component)
    {}

    /**
     * Guess the view name for the given component.
     *
     * @param  string  $name
     * @param  string  $prefix
     * @return string
     */
    public function guessViewName($name, $prefix = 'components.')
    {}

    /**
     * Partition the data and extra attributes from the given array of attributes.
     *
     * @param  string  $class
     * @param  array  $attributes
     * @return array
     */
    public function partitionDataAndAttributes($class, array $attributes)
    {}

    /**
     * Compile the closing tags within the given string.
     *
     * @param  string  $value
     * @return string
     */
    protected function compileClosingTags(string $value)
    {}

    /**
     * Compile the slot tags within the given string.
     *
     * @param  string  $value
     * @return string
     */
    public function compileSlots(string $value)
    {}

    /**
     * Get an array of attributes from the given attribute string.
     *
     * @param  string  $attributeString
     * @return array
     */
    protected function getAttributesFromAttributeString(string $attributeString)
    {}

    /**
     * Parses a short attribute syntax like :$foo into a fully-qualified syntax like :foo="$foo".
     *
     * @param  string  $value
     * @return string
     */
    protected function parseShortAttributeSyntax(string $value)
    {}

    /**
     * Parse the attribute bag in a given attribute string into its fully-qualified syntax.
     *
     * @param  string  $attributeString
     * @return string
     */
    protected function parseAttributeBag(string $attributeString)
    {}

    /**
     * Parse @class statements in a given attribute string into their fully-qualified syntax.
     *
     * @param  string  $attributeString
     * @return string
     */
    protected function parseComponentTagClassStatements(string $attributeString)
    {}

    /**
     * Parse @style statements in a given attribute string into their fully-qualified syntax.
     *
     * @param  string  $attributeString
     * @return string
     */
    protected function parseComponentTagStyleStatements(string $attributeString)
    {}

    /**
     * Parse the "bind" attributes in a given attribute string into their fully-qualified syntax.
     *
     * @param  string  $attributeString
     * @return string
     */
    protected function parseBindAttributes(string $attributeString)
    {}

    /**
     * Compile any Blade echo statements that are present in the attribute string.
     *
     * These echo statements need to be converted to string concatenation statements.
     *
     * @param  string  $attributeString
     * @return string
     */
    protected function compileAttributeEchos(string $attributeString)
    {}

    /**
     * Escape the single quotes in the given string that are outside of PHP blocks.
     *
     * @param  string  $value
     * @return string
     */
    protected function escapeSingleQuotesOutsideOfPhpBlocks(string $value)
    {}

    /**
     * Convert an array of attributes to a string.
     *
     * @param  array  $attributes
     * @param  bool  $escapeBound
     * @return string
     */
    protected function attributesToString(array $attributes, $escapeBound = true)
    {}

    /**
     * Strip any quotes from the given string.
     *
     * @param  string  $value
     * @return string
     */
    public function stripQuotes(string $value)
    {}
}
