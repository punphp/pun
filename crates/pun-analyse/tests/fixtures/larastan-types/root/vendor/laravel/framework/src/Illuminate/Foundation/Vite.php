<?php

namespace Illuminate\Foundation;

use Illuminate\Contracts\Support\Htmlable;
use Illuminate\Support\Collection;
use Illuminate\Support\HtmlString;
use Illuminate\Support\Js;
use Illuminate\Support\Str;
use Illuminate\Support\Traits\Macroable;

class Vite implements Htmlable
{
    use Macroable;

    /**
     * The Content Security Policy nonce to apply to all generated tags.
     *
     * @var string|null
     */
    protected $nonce;

    /**
     * The key to check for integrity hashes within the manifest.
     *
     * @var string|false
     */
    protected $integrityKey = 'integrity';

    /**
     * The configured entry points.
     *
     * @var array
     */
    protected $entryPoints = [];

    /**
     * The path to the "hot" file.
     *
     * @var string|null
     */
    protected $hotFile;

    /**
     * The path to the build directory.
     *
     * @var string
     */
    protected $buildDirectory = 'build';

    /**
     * The name of the manifest file.
     *
     * @var string
     */
    protected $manifestFilename = 'manifest.json';

    /**
     * The custom asset path resolver.
     *
     * @var callable|null
     */
    protected $assetPathResolver = null;

    /**
     * The script tag attributes resolvers.
     *
     * @var array
     */
    protected $scriptTagAttributesResolvers = [];

    /**
     * The style tag attributes resolvers.
     *
     * @var array
     */
    protected $styleTagAttributesResolvers = [];

    /**
     * The preload tag attributes resolvers.
     *
     * @var array
     */
    protected $preloadTagAttributesResolvers = [];

    /**
     * The preloaded assets.
     *
     * @var array
     */
    protected $preloadedAssets = [];

    /**
     * The cached manifest files.
     *
     * @var array
     */
    protected static $manifests = [];

    /**
     * The ViteFonts instance.
     *
     * @var \Illuminate\Foundation\ViteFonts|null
     */
    protected $fonts = null;

    /**
     * The name of the font manifest file.
     *
     * @var string
     */
    protected $fontsManifestFilename = 'fonts-manifest.json';

    /**
     * The prefetching strategy to use.
     *
     * @var null|'waterfall'|'aggressive'
     */
    protected $prefetchStrategy = null;

    /**
     * The number of assets to load concurrently when using the "waterfall" strategy.
     *
     * @var int
     */
    protected $prefetchConcurrently = 3;

    /**
     * The name of the event that should trigger prefetching. The event must be dispatched on the `window`.
     *
     * @var string
     */
    protected $prefetchEvent = 'load';

    /**
     * Get the preloaded assets.
     *
     * @return array
     */
    public function preloadedAssets()
    {}

    /**
     * Get the Content Security Policy nonce applied to all generated tags.
     *
     * @return string|null
     */
    public function cspNonce()
    {}

    /**
     * Generate or set a Content Security Policy nonce to apply to all generated tags.
     *
     * @param  string|null  $nonce
     * @return string
     */
    public function useCspNonce($nonce = null)
    {}

    /**
     * Use the given key to detect integrity hashes in the manifest.
     *
     * @param  string|false  $key
     * @return $this
     */
    public function useIntegrityKey($key)
    {}

    /**
     * Set the Vite entry points.
     *
     * @param  array  $entryPoints
     * @return $this
     */
    public function withEntryPoints($entryPoints)
    {}

    /**
     * Merge additional Vite entry points with the current set.
     *
     * @param  array  $entryPoints
     * @return $this
     */
    public function mergeEntryPoints($entryPoints)
    {}

    /**
     * Set the filename for the manifest file.
     *
     * @param  string  $filename
     * @return $this
     */
    public function useManifestFilename($filename)
    {}

    /**
     * Resolve asset paths using the provided resolver.
     *
     * @param  callable|null  $resolver
     * @return $this
     */
    public function createAssetPathsUsing($resolver)
    {}

    /**
     * Get the Vite "hot" file path.
     *
     * @return string
     */
    public function hotFile()
    {}

    /**
     * Set the Vite "hot" file path.
     *
     * @param  string  $path
     * @return $this
     */
    public function useHotFile($path)
    {}

    /**
     * Set the Vite build directory.
     *
     * @param  string  $path
     * @return $this
     */
    public function useBuildDirectory($path)
    {}

    /**
     * Use the given callback to resolve attributes for script tags.
     *
     * @param  (callable(string, string, ?array, ?array): array)|array  $attributes
     * @return $this
     */
    public function useScriptTagAttributes($attributes)
    {}

    /**
     * Use the given callback to resolve attributes for style tags.
     *
     * @param  (callable(string, string, ?array, ?array): array)|array  $attributes
     * @return $this
     */
    public function useStyleTagAttributes($attributes)
    {}

    /**
     * Use the given callback to resolve attributes for preload tags.
     *
     * @param  (callable(string, string, ?array, ?array): (array|false))|array|false  $attributes
     * @return $this
     */
    public function usePreloadTagAttributes($attributes)
    {}

    /**
     * Eagerly prefetch assets.
     *
     * @param  int|null  $concurrency
     * @param  string  $event
     * @return $this
     */
    public function prefetch($concurrency = null, $event = 'load')
    {}

    /**
     * Use the "waterfall" prefetching strategy.
     *
     * @return $this
     */
    public function useWaterfallPrefetching(?int $concurrency = null)
    {}

    /**
     * Use the "aggressive" prefetching strategy.
     *
     * @return $this
     */
    public function useAggressivePrefetching()
    {}

    /**
     * Set the prefetching strategy.
     *
     * @param  'waterfall'|'aggressive'|null  $strategy
     * @param  array  $config
     * @return $this
     */
    public function usePrefetchStrategy($strategy, $config = [])
    {}

    /**
     * Generate Vite tags for an entrypoint.
     *
     * @param  string|string[]  $entrypoints
     * @param  string|null  $buildDirectory
     * @return \Illuminate\Support\HtmlString
     *
     * @throws \Exception
     */
    public function __invoke($entrypoints, $buildDirectory = null)
    {}

    /**
     * Make tag for the given chunk.
     *
     * @param  string  $src
     * @param  string  $url
     * @param  array|null  $chunk
     * @param  array|null  $manifest
     * @return string
     */
    protected function makeTagForChunk($src, $url, $chunk, $manifest)
    {}

    /**
     * Make a preload tag for the given chunk.
     *
     * @param  string  $src
     * @param  string  $url
     * @param  array  $chunk
     * @param  array  $manifest
     * @return string
     */
    protected function makePreloadTagForChunk($src, $url, $chunk, $manifest)
    {}

    /**
     * Resolve the attributes for the chunks generated script tag.
     *
     * @param  string  $src
     * @param  string  $url
     * @param  array|null  $chunk
     * @param  array|null  $manifest
     * @return array
     */
    protected function resolveScriptTagAttributes($src, $url, $chunk, $manifest)
    {}

    /**
     * Resolve the attributes for the chunks generated stylesheet tag.
     *
     * @param  string  $src
     * @param  string  $url
     * @param  array|null  $chunk
     * @param  array|null  $manifest
     * @return array
     */
    protected function resolveStylesheetTagAttributes($src, $url, $chunk, $manifest)
    {}

    /**
     * Resolve the attributes for the chunks generated preload tag.
     *
     * @param  string  $src
     * @param  string  $url
     * @param  array  $chunk
     * @param  array  $manifest
     * @return array|false
     */
    protected function resolvePreloadTagAttributes($src, $url, $chunk, $manifest)
    {}

    /**
     * Generate an appropriate tag for the given URL in HMR mode.
     *
     * @deprecated Will be removed in a future Laravel version.
     *
     * @param  string  $url
     * @return string
     */
    protected function makeTag($url)
    {}

    /**
     * Generate a script tag for the given URL.
     *
     * @deprecated Will be removed in a future Laravel version.
     *
     * @param  string  $url
     * @return string
     */
    protected function makeScriptTag($url)
    {}

    /**
     * Generate a stylesheet tag for the given URL in HMR mode.
     *
     * @deprecated Will be removed in a future Laravel version.
     *
     * @param  string  $url
     * @return string
     */
    protected function makeStylesheetTag($url)
    {}

    /**
     * Generate a script tag with attributes for the given URL.
     *
     * @param  string  $url
     * @param  array  $attributes
     * @return string
     */
    protected function makeScriptTagWithAttributes($url, $attributes)
    {}

    /**
     * Generate a link tag with attributes for the given URL.
     *
     * @param  string  $url
     * @param  array  $attributes
     * @return string
     */
    protected function makeStylesheetTagWithAttributes($url, $attributes)
    {}

    /**
     * Determine whether the given path is a CSS file.
     *
     * @param  string  $path
     * @return bool
     */
    protected function isCssPath($path)
    {}

    /**
     * Parse the attributes into key="value" strings.
     *
     * @param  array  $attributes
     * @return array
     */
    protected function parseAttributes($attributes)
    {}

    /**
     * Generate React refresh runtime script.
     *
     * @return \Illuminate\Support\HtmlString|void
     */
    public function reactRefresh()
    {}

    /**
     * Get the path to a given asset when running in HMR mode.
     *
     * @return string
     */
    protected function hotAsset($asset)
    {}

    /**
     * Get the URL of the running Vite development server.
     *
     * @return string|null
     */
    public function devServerUrl()
    {}

    /**
     * Get the URL for an asset.
     *
     * @param  string  $asset
     * @param  string|null  $buildDirectory
     * @return string
     */
    public function asset($asset, $buildDirectory = null)
    {}

    /**
     * Get the content of a given asset.
     *
     * @param  string  $asset
     * @param  string|null  $buildDirectory
     * @return string
     *
     * @throws \Illuminate\Foundation\ViteException
     */
    public function content($asset, $buildDirectory = null)
    {}

    /**
     * Generate an asset path for the application.
     *
     * @param  string  $path
     * @param  bool|null  $secure
     * @return string
     */
    protected function assetPath($path, $secure = null)
    {}

    /**
     * Generate a public path for an asset.
     *
     * @param  string  $path
     * @return string
     */
    protected function publicPath($path)
    {}

    /**
     * Get the manifest file for the given build directory.
     *
     * @param  string  $buildDirectory
     * @return array
     *
     * @throws \Illuminate\Foundation\ViteManifestNotFoundException
     */
    protected function manifest($buildDirectory)
    {}

    /**
     * Get the path to the manifest file for the given build directory.
     *
     * @param  string  $buildDirectory
     * @return string
     */
    protected function manifestPath($buildDirectory)
    {}

    /**
     * Get a unique hash representing the current manifest, or null if there is no manifest.
     *
     * @param  string|null  $buildDirectory
     * @return string|null
     */
    public function manifestHash($buildDirectory = null)
    {}

    /**
     * Recursively resolve all imports for the given chunk.
     *
     * @param  array  $manifest
     * @param  array  $chunk
     * @param  array  $seen
     * @return array
     */
    protected function resolveImports($manifest, $chunk, $seen = [])
    {}

    /**
     * Get the chunk for the given entry point / asset.
     *
     * @param  array  $manifest
     * @param  string  $file
     * @return array
     *
     * @throws \Illuminate\Foundation\ViteException
     */
    protected function chunk($manifest, $file)
    {}

    /**
     * Get the nonce attribute for the prefetch script tags.
     *
     * @return \Illuminate\Support\HtmlString
     */
    protected function nonceAttribute()
    {}

    /**
     * Render font preload links and inline styles.
     *
     * @param  list<string>|string|null  $aliases
     * @return \Illuminate\Support\HtmlString
     *
     * @throws \Illuminate\Foundation\ViteException
     */
    public function fonts($aliases = null)
    {}

    /**
     * Render preload link tags for font entries.
     *
     * @param  list<array<string, string>>  $preloads
     * @return string
     */
    protected function renderFontPreloads($preloads)
    {}

    /**
     * Resolve the attributes for a font preload tag.
     *
     * @param  string  $url
     * @param  array<string, string>  $preload
     * @return array<string, string|false>|false
     */
    protected function resolveFontPreloadAttributes($url, $preload)
    {}

    /**
     * Render the inline style block for the font manifest.
     *
     * @param  array<string, mixed>  $manifest
     * @param  list<string>|null  $aliases
     * @return string
     */
    protected function renderFontStyle($manifest, $aliases)
    {}

    /**
     * Get the ViteFonts instance.
     *
     * @return \Illuminate\Foundation\ViteFonts
     */
    protected function viteFonts()
    {}

    /**
     * Set the font manifest filename.
     *
     * @param  string  $filename
     * @return $this
     */
    public function useFontsManifestFilename($filename)
    {}

    /**
     * Determine if the HMR server is running.
     *
     * @return bool
     */
    public function isRunningHot()
    {}

    /**
     * Get the Vite tag content as a string of HTML.
     *
     * @return string
     */
    public function toHtml()
    {}

    /**
     * Flush state.
     *
     * @return void
     */
    public function flush()
    {}
}
