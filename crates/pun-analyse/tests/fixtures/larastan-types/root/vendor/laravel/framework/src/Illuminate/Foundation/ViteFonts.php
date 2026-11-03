<?php

namespace Illuminate\Foundation;

class ViteFonts
{
    /**
     * The cached font manifests.
     *
     * @var array<string, array<string, mixed>>
     */
    protected static $manifests = [];

    /**
     * Read the font manifest for the given configuration.
     *
     * @param  bool  $isHot
     * @param  string  $buildDirectory
     * @param  string  $manifestFilename
     * @param  string  $hotFile
     * @return array<string, mixed>|null
     *
     * @throws \Illuminate\Foundation\ViteException
     */
    public function manifest(bool $isHot, string $buildDirectory, string $manifestFilename, string $hotFile)
    {}

    /**
     * Read and decode a manifest file.
     *
     * @param  string  $path
     * @return array<string, mixed>|null
     *
     * @throws \Illuminate\Foundation\ViteException
     */
    protected function readManifest(string $path)
    {}

    /**
     * Resolve the CSS content from the manifest.
     *
     * @param  array<string, mixed>  $manifest
     * @param  list<string>|null  $aliases
     * @param  string  $buildDirectory
     * @return string
     *
     * @throws \Illuminate\Foundation\ViteException
     */
    public function resolveStyleContent(array $manifest, ?array $aliases, string $buildDirectory)
    {}

    /**
     * Resolve filtered CSS content using per-alias fragments from the manifest.
     *
     * @param  array{inline?: string, file?: string, familyStyles?: array<string, string>, variables?: array<string, string>}  $style
     * @param  list<string>  $aliases
     * @return string
     *
     * @throws \Illuminate\Foundation\ViteException
     */
    protected function resolveFilteredStyleContent(array $style, array $aliases)
    {}

    /**
     * Build a `:root` block containing only the CSS variables for the given aliases.
     *
     * @param  array<string, string>  $variables
     * @param  list<string>  $aliases  List of aliases in desired emission order.
     * @return string
     */
    protected function filterVariables(array $variables, array $aliases)
    {}

    /**
     * Read a CSS file from the build directory.
     *
     * @param  string  $buildDirectory
     * @param  string  $file
     * @return string
     *
     * @throws \Illuminate\Foundation\ViteException
     */
    protected function readStyleFile(string $buildDirectory, string $file)
    {}

    /**
     * Validate the font manifest structure.
     *
     * @param  array<string, mixed>  $manifest
     * @return void
     *
     * @throws \Illuminate\Foundation\ViteException
     */
    public function ensureValidManifest(array $manifest)
    {}

    /**
     * Validate that the requested aliases exist in the manifest.
     *
     * @param  list<string>  $aliases
     * @param  array<string, mixed>  $manifest
     * @return void
     *
     * @throws \Illuminate\Foundation\ViteException
     */
    public function ensureValidFamilies(array $aliases, array $manifest)
    {}

    /**
     * Validate that each preload entry contains the required keys.
     *
     * @param  list<array<string, string>>  $preloads
     * @param  bool  $isHot
     * @return void
     *
     * @throws \Illuminate\Foundation\ViteException
     */
    public function ensureValidPreloads(array $preloads, bool $isHot)
    {}

    /**
     * Flush cached manifests.
     *
     * @return void
     */
    public function flush()
    {}
}
