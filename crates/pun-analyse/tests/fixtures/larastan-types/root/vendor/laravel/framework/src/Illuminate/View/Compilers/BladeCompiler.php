<?php

namespace Illuminate\View\Compilers;

use Illuminate\Container\Container;
use Illuminate\Contracts\Support\Htmlable;
use Illuminate\Contracts\View\Factory as ViewFactory;
use Illuminate\Contracts\View\View;
use Illuminate\Support\Arr;
use Illuminate\Support\Collection;
use Illuminate\Support\Str;
use Illuminate\Support\Stringable;
use Illuminate\Support\Traits\ReflectsClosures;
use Illuminate\View\Component;
use InvalidArgumentException;
use ParseError;

class BladeCompiler extends Compiler implements CompilerInterface
{
    use Concerns\CompilesAuthorizations,
        Concerns\CompilesClasses,
        Concerns\CompilesComments,
        Concerns\CompilesComponents,
        Concerns\CompilesConditionals,
        Concerns\CompilesContexts,
        Concerns\CompilesEchos,
        Concerns\CompilesErrors,
        Concerns\CompilesFragments,
        Concerns\CompilesHelpers,
        Concerns\CompilesIncludes,
        Concerns\CompilesInjections,
        Concerns\CompilesJson,
        Concerns\CompilesJs,
        Concerns\CompilesLayouts,
        Concerns\CompilesLoops,
        Concerns\CompilesRawPhp,
        Concerns\CompilesSessions,
        Concerns\CompilesStacks,
        Concerns\CompilesStyles,
        Concerns\CompilesTranslations,
        Concerns\CompilesUseStatements,
        ReflectsClosures;

    const DEFAULT_ECHO_FORMAT = 'e(%s)';

    /**
     * All of the registered extensions.
     *
     * @var array
     */
    protected $extensions = [];

    /**
     * All custom "directive" handlers.
     *
     * @var array
     */
    protected $customDirectives = [];

    /**
     * All custom "condition" handlers.
     *
     * @var array
     */
    protected $conditions = [];

    /**
     * The registered string preparation callbacks.
     *
     * @var array
     */
    protected $prepareStringsForCompilationUsing = [];

    /**
     * All of the registered precompilers.
     *
     * @var array
     */
    protected $precompilers = [];

    /**
     * The file currently being compiled.
     *
     * @var string
     */
    protected $path;

    /**
     * All of the available compiler functions.
     *
     * @var string[]
     */
    protected $compilers = [
        // 'Comments',
        'Extensions',
        'Statements',
        'Echos',
    ];

    /**
     * Array of opening and closing tags for raw echos.
     *
     * @var string[]
     */
    protected $rawTags = ['{!!', '!!}'];

    /**
     * Array of opening and closing tags for regular echos.
     *
     * @var string[]
     */
    protected $contentTags = ['{{', '}}'];

    /**
     * Array of opening and closing tags for escaped echos.
     *
     * @var string[]
     */
    protected $escapedTags = ['{{{', '}}}'];

    /**
     * The "regular" / legacy echo string format.
     *
     * @var string
     */
    protected $echoFormat = self::DEFAULT_ECHO_FORMAT;

    /**
     * Array of footer lines to be added to the template.
     *
     * @var array
     */
    protected $footer = [];

    /**
     * Array to temporarily store the raw blocks found in the template.
     *
     * @var array
     */
    protected $rawBlocks = [];

    /**
     * The array of anonymous component paths to search for components in.
     *
     * @var array
     */
    protected $anonymousComponentPaths = [];

    /**
     * The array of anonymous component namespaces to autoload from.
     *
     * @var array
     */
    protected $anonymousComponentNamespaces = [];

    /**
     * The array of class component aliases and their class names.
     *
     * @var array
     */
    protected $classComponentAliases = [];

    /**
     * The array of class component namespaces to autoload from.
     *
     * @var array
     */
    protected $classComponentNamespaces = [];

    /**
     * Indicates if component tags should be compiled.
     *
     * @var bool
     */
    protected $compilesComponentTags = true;

    /**
     * Compile the view at the given path.
     *
     * @param  string|null  $path
     * @return void
     */
    public function compile($path = null)
    {}

    /**
     * Append the file path to the compiled string.
     *
     * @param  string  $contents
     * @return string
     */
    protected function appendFilePath($contents)
    {}

    /**
     * Get the open and closing PHP tag tokens from the given string.
     *
     * @param  string  $contents
     * @return \Illuminate\Support\Collection
     */
    protected function getOpenAndClosingPhpTokens($contents)
    {}

    /**
     * Get the path currently being compiled.
     *
     * @return string
     */
    public function getPath()
    {}

    /**
     * Set the path currently being compiled.
     *
     * @param  string  $path
     * @return void
     */
    public function setPath($path)
    {}

    /**
     * Compile the given Blade template contents.
     *
     * @param  string  $value
     * @return string
     */
    public function compileString($value)
    {}

    /**
     * Evaluate and render a Blade string to HTML.
     *
     * @param  string  $string
     * @param  array  $data
     * @param  bool  $deleteCachedView
     * @return string
     */
    public static function render($string, $data = [], $deleteCachedView = false)
    {}

    /**
     * Render a component instance to HTML.
     *
     * @param  \Illuminate\View\Component  $component
     * @return string
     */
    public static function renderComponent(Component $component)
    {}

    /**
     * Store the blocks that do not receive compilation.
     *
     * @param  string  $value
     * @return string
     */
    protected function storeUncompiledBlocks($value)
    {}

    /**
     * Store the verbatim blocks and replace them with a temporary placeholder.
     *
     * @param  string  $value
     * @return string
     */
    protected function storeVerbatimBlocks($value)
    {}

    /**
     * Store the PHP blocks and replace them with a temporary placeholder.
     *
     * @param  string  $value
     * @return string
     */
    protected function storePhpBlocks($value)
    {}

    /**
     * Store a raw block and return a unique raw placeholder.
     *
     * @param  string  $value
     * @return string
     */
    protected function storeRawBlock($value)
    {}

    /**
     * Compile the component tags.
     *
     * @param  string  $value
     * @return string
     */
    protected function compileComponentTags($value)
    {}

    /**
     * Replace the raw placeholders with the original code stored in the raw blocks.
     *
     * @param  string  $result
     * @return string
     */
    protected function restoreRawContent($result)
    {}

    /**
     * Get a placeholder to temporarily mark the position of raw blocks.
     *
     * @param  int|string  $replace
     * @return string
     */
    protected function getRawPlaceholder($replace)
    {}

    /**
     * Add the stored footers onto the given content.
     *
     * @param  string  $result
     * @return string
     */
    protected function addFooters($result)
    {}

    /**
     * Parse the tokens from the template.
     *
     * @param  array  $token
     * @return string
     */
    protected function parseToken($token)
    {}

    /**
     * Execute the user defined extensions.
     *
     * @param  string  $value
     * @return string
     */
    protected function compileExtensions($value)
    {}

    /**
     * Compile Blade statements that start with "@".
     *
     * @param  string  $template
     * @return string
     */
    protected function compileStatements($template)
    {}

    /**
     * Replace the first match for a statement compilation operation.
     *
     * @param  string  $search
     * @param  string  $replace
     * @param  string  $subject
     * @param  int  $offset
     * @return array
     */
    protected function replaceFirstStatement($search, $replace, $subject, $offset)
    {}

    /**
     * Determine if the given expression has the same number of opening and closing parentheses.
     *
     * @param  string  $expression
     * @return bool
     */
    protected function hasEvenNumberOfParentheses(string $expression)
    {}

    /**
     * Compile a single Blade @ statement.
     *
     * @param  array  $match
     * @return string
     */
    protected function compileStatement($match)
    {}

    /**
     * Call the given directive with the given value.
     *
     * @param  string  $name
     * @param  string|null  $value
     * @return string
     */
    protected function callCustomDirective($name, $value)
    {}

    /**
     * Strip the parentheses from the given expression.
     *
     * @param  string  $expression
     * @return string
     */
    public function stripParentheses($expression)
    {}

    /**
     * Register a custom Blade compiler.
     *
     * @param  callable  $compiler
     * @return void
     */
    public function extend(callable $compiler)
    {}

    /**
     * Get the extensions used by the compiler.
     *
     * @return array
     */
    public function getExtensions()
    {}

    /**
     * Register an "if" statement directive.
     *
     * @param  string  $name
     * @param  callable  $callback
     * @return void
     */
    public function if($name, callable $callback)
    {}

    /**
     * Check the result of a condition.
     *
     * @param  string  $name
     * @param  mixed  ...$parameters
     * @return bool
     */
    public function check($name, ...$parameters)
    {}

    /**
     * Register a class-based component alias directive.
     *
     * @param  string  $class
     * @param  string|null  $alias
     * @param  string  $prefix
     * @return void
     */
    public function component($class, $alias = null, $prefix = '')
    {}

    /**
     * Register an array of class-based components.
     *
     * @param  array  $components
     * @param  string  $prefix
     * @return void
     */
    public function components(array $components, $prefix = '')
    {}

    /**
     * Get the registered class component aliases.
     *
     * @return array
     */
    public function getClassComponentAliases()
    {}

    /**
     * Register a new anonymous component path.
     *
     * @param  string  $path
     * @param  string|null  $prefix
     * @return void
     */
    public function anonymousComponentPath(string $path, ?string $prefix = null)
    {}

    /**
     * Register an anonymous component namespace.
     *
     * @param  string  $directory
     * @param  string|null  $prefix
     * @return void
     */
    public function anonymousComponentNamespace(string $directory, ?string $prefix = null)
    {}

    /**
     * Register a class-based component namespace.
     *
     * @param  string  $namespace
     * @param  string  $prefix
     * @return void
     */
    public function componentNamespace($namespace, $prefix)
    {}

    /**
     * Get the registered anonymous component paths.
     *
     * @return array
     */
    public function getAnonymousComponentPaths()
    {}

    /**
     * Get the registered anonymous component namespaces.
     *
     * @return array
     */
    public function getAnonymousComponentNamespaces()
    {}

    /**
     * Get the registered class component namespaces.
     *
     * @return array
     */
    public function getClassComponentNamespaces()
    {}

    /**
     * Register a component alias directive.
     *
     * @param  string  $path
     * @param  string|null  $alias
     * @return void
     */
    public function aliasComponent($path, $alias = null)
    {}

    /**
     * Register an include alias directive.
     *
     * @param  string  $path
     * @param  string|null  $alias
     * @return void
     */
    public function include($path, $alias = null)
    {}

    /**
     * Register an include alias directive.
     *
     * @param  string  $path
     * @param  string|null  $alias
     * @return void
     */
    public function aliasInclude($path, $alias = null)
    {}

    /**
     * Register a handler for custom directives, binding the handler to the compiler.
     *
     * @param  string  $name
     * @param  callable  $handler
     * @return void
     *
     * @throws \InvalidArgumentException
     */
    public function bindDirective($name, callable $handler)
    {}

    /**
     * Register a handler for custom directives.
     *
     * @param  string  $name
     * @param  ($bind is true ? \Closure : callable)  $handler
     * @param  bool  $bind
     * @return void
     *
     * @throws \InvalidArgumentException
     */
    public function directive($name, callable $handler, bool $bind = false)
    {}

    /**
     * Get the list of custom directives.
     *
     * @return array
     */
    public function getCustomDirectives()
    {}

    /**
     * Indicate that the following callable should be used to prepare strings for compilation.
     *
     * @param  callable  $callback
     * @return $this
     */
    public function prepareStringsForCompilationUsing(callable $callback)
    {}

    /**
     * Register a new precompiler.
     *
     * @param  callable  $precompiler
     * @return void
     */
    public function precompiler(callable $precompiler)
    {}

    /**
     * Get the path to the compiled version of a view.
     *
     * @param  string  $path
     * @return string
     */
    public function getCompiledPath($path)
    {}

    /**
     * Execute the given callback using a custom echo format.
     *
     * @param  string  $format
     * @param  callable  $callback
     * @return string
     */
    public function usingEchoFormat($format, callable $callback)
    {}

    /**
     * Set the echo format to be used by the compiler.
     *
     * @param  string  $format
     * @return void
     */
    public function setEchoFormat($format)
    {}

    /**
     * Set the "echo" format to double encode entities.
     *
     * @return void
     */
    public function withDoubleEncoding()
    {}

    /**
     * Set the "echo" format to not double encode entities.
     *
     * @return void
     */
    public function withoutDoubleEncoding()
    {}

    /**
     * Indicate that component tags should not be compiled.
     *
     * @return void
     */
    public function withoutComponentTags()
    {}
}
