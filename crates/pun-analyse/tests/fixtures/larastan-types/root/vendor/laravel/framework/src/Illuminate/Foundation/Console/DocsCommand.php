<?php

namespace Illuminate\Foundation\Console;

use Carbon\CarbonInterval;
use Illuminate\Console\Command;
use Illuminate\Contracts\Cache\Repository as Cache;
use Illuminate\Http\Client\Factory as Http;
use Illuminate\Support\Arr;
use Illuminate\Support\Collection;
use Illuminate\Support\Env;
use Illuminate\Support\Str;
use Symfony\Component\Console\Attribute\AsCommand;
use Symfony\Component\Process\Exception\ProcessFailedException;
use Symfony\Component\Process\ExecutableFinder;
use Symfony\Component\Process\Process;
use Throwable;

use function Laravel\Prompts\suggest;

#[AsCommand(name: 'docs')]
class DocsCommand extends Command
{
    /**
     * The name and signature of the console command.
     *
     * @var string
     */
    protected $signature = 'docs {page? : The documentation page to open} {section? : The section of the page to open}';

    /**
     * The console command description.
     *
     * @var string
     */
    protected $description = 'Access the Laravel documentation';

    /**
     * The console command help text.
     *
     * @var string
     */
    protected $help = 'If you would like to perform a content search against the documentation, you may call: <fg=green>php artisan docs -- </><fg=green;options=bold;>search query here</>';

    /**
     * The HTTP client instance.
     *
     * @var \Illuminate\Http\Client\Factory
     */
    protected $http;

    /**
     * The cache repository implementation.
     *
     * @var \Illuminate\Contracts\Cache\Repository
     */
    protected $cache;

    /**
     * The custom URL opener.
     *
     * @var callable|null
     */
    protected $urlOpener;

    /**
     * The custom documentation version to open.
     *
     * @var string|null
     */
    protected $version;

    /**
     * The operating system family.
     *
     * @var string
     */
    protected $systemOsFamily = PHP_OS_FAMILY;

    /**
     * Configure the current command.
     *
     * @return void
     */
    protected function configure(): void
    {}

    /**
     * Execute the console command.
     *
     * @param  \Illuminate\Http\Client\Factory  $http
     * @param  \Illuminate\Contracts\Cache\Repository  $cache
     * @return int
     *
     * @throws \Symfony\Component\Process\Exception\ProcessFailedException
     */
    public function handle(Http $http, Cache $cache)
    {}

    /**
     * Open the documentation URL.
     *
     * @return void
     */
    protected function openUrl()
    {}

    /**
     * The URL to the documentation page.
     *
     * @return string
     */
    protected function url()
    {}

    /**
     * The page the user is opening.
     *
     * @return string
     */
    protected function page()
    {}

    /**
     * Determine the page to open.
     *
     * @return string|null
     */
    protected function resolvePage()
    {}

    /**
     * Determine if the user requested a specific page when calling the command.
     *
     * @return bool
     */
    protected function didNotRequestPage()
    {}

    /**
     * Ask the user which page they would like to open.
     *
     * @return string|null
     */
    protected function askForPage()
    {}

    /**
     * Ask the user which page they would like to open via a custom strategy.
     *
     * @return string|null
     */
    protected function askForPageViaCustomStrategy()
    {}

    /**
     * Ask the user which page they would like to open using autocomplete.
     *
     * @return string|null
     */
    protected function askForPageViaAutocomplete()
    {}

    /**
     * Guess the page the user is attempting to open.
     *
     * @return string|null
     */
    protected function guessPage($search)
    {}

    /**
     * The section the user specifically asked to open.
     *
     * @param  string  $page
     * @return string|null
     */
    protected function section($page)
    {}

    /**
     * Determine if the user requested a specific section when calling the command.
     *
     * @return bool
     */
    protected function didNotRequestSection()
    {}

    /**
     * Guess the section the user is attempting to open.
     *
     * @param  string  $page
     * @return string|null
     */
    protected function guessSection($page)
    {}

    /**
     * Open the URL in the user's browser.
     *
     * @param  string  $url
     * @return void
     */
    protected function open($url)
    {}

    /**
     * Open the URL via a custom strategy.
     *
     * @param  string  $url
     * @return void
     */
    protected function openViaCustomStrategy($url)
    {}

    /**
     * Open the URL via the built in strategy.
     *
     * @param  string  $url
     * @return void
     *
     * @throws \Symfony\Component\Process\Exception\ProcessFailedException
     */
    protected function openViaBuiltInStrategy($url)
    {}

    /**
     * The available sections for the page.
     *
     * @param  string  $page
     * @return \Illuminate\Support\Collection
     */
    public function sectionsFor($page)
    {}

    /**
     * The pages available to open.
     *
     * @return \Illuminate\Support\Collection
     */
    public function pages()
    {}

    /**
     * Get the documentation index as a collection.
     *
     * @return \Illuminate\Support\Collection
     */
    public function docs()
    {}

    /**
     * Refresh the cached copy of the documentation index.
     *
     * @return void
     */
    protected function refreshDocs()
    {}

    /**
     * Fetch the documentation index from the Laravel website.
     *
     * @return \Illuminate\Http\Client\Response
     */
    protected function fetchDocs()
    {}

    /**
     * Determine the version of the docs to open.
     *
     * @return string
     */
    protected function version()
    {}

    /**
     * The search query the user provided.
     *
     * @return string
     */
    protected function searchQuery()
    {}

    /**
     * Determine if the command is intended to perform a search.
     *
     * @return bool
     */
    protected function isSearching()
    {}

    /**
     * Set the documentation version.
     *
     * @param  string  $version
     * @return $this
     */
    public function setVersion($version)
    {}

    /**
     * Set a custom URL opener.
     *
     * @param  callable|null  $opener
     * @return $this
     */
    public function setUrlOpener($opener)
    {}

    /**
     * Set the system operating system family.
     *
     * @param  string  $family
     * @return $this
     */
    public function setSystemOsFamily($family)
    {}
}
