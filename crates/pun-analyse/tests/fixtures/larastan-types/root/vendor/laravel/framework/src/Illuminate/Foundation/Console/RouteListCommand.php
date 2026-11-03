<?php

namespace Illuminate\Foundation\Console;

use Closure;
use Illuminate\Console\Command;
use Illuminate\Contracts\Routing\UrlGenerator;
use Illuminate\Routing\Route;
use Illuminate\Routing\Router;
use Illuminate\Routing\ViewController;
use Illuminate\Support\Arr;
use Illuminate\Support\Collection;
use Illuminate\Support\Str;
use Illuminate\Support\Stringable;
use ReflectionClass;
use ReflectionFunction;
use Symfony\Component\Console\Attribute\AsCommand;
use Symfony\Component\Console\Terminal;

#[AsCommand(name: 'route:list')]
class RouteListCommand extends Command
{
    /**
     * The name and signature of the console command.
     *
     * @var string
     */
    protected $signature = 'route:list
                    {--json : Output the route list as JSON}
                    {--method= : Filter the routes by method}
                    {--action= : Filter the routes by action}
                    {--name= : Filter the routes by name}
                    {--domain= : Filter the routes by domain}
                    {--middleware= : Filter the routes by middleware}
                    {--path= : Only show routes matching the given path pattern}
                    {--except-path= : Do not display the routes matching the given path pattern}
                    {--r|reverse : Reverse the ordering of the routes}
                    {--sort=uri : The column (domain, method, uri, name, action, middleware, definition) to sort by}
                    {--except-vendor : Do not display routes defined by vendor packages}
                    {--only-vendor : Only display routes defined by vendor packages}';

    /**
     * The console command description.
     *
     * @var string
     */
    protected $description = 'List all registered routes';

    /**
     * The router instance.
     *
     * @var \Illuminate\Routing\Router
     */
    protected $router;

    /**
     * The table headers for the command.
     *
     * @var string[]
     */
    protected $headers = ['Domain', 'Method', 'URI', 'Name', 'Action', 'Middleware', 'Path'];

    /**
     * The terminal width resolver callback.
     *
     * @var \Closure|null
     */
    protected static $terminalWidthResolver;

    /**
     * The verb colors for the command.
     *
     * @var array
     */
    protected $verbColors = [
        'ANY' => 'red',
        'GET' => 'blue',
        'HEAD' => '#6C7280',
        'OPTIONS' => '#6C7280',
        'QUERY' => '#6C7280',
        'POST' => 'yellow',
        'PUT' => 'yellow',
        'PATCH' => 'yellow',
        'DELETE' => 'red',
    ];

    /**
     * Create a new route command instance.
     *
     * @param  \Illuminate\Routing\Router  $router
     */
    public function __construct(Router $router)
    {}

    /**
     * Execute the console command.
     *
     * @return void
     */
    public function handle()
    {}

    /**
     * Compile the routes into a displayable format.
     *
     * @return array
     */
    protected function getRoutes()
    {}

    /**
     * Get the route information for a given route.
     *
     * @param  \Illuminate\Routing\Route  $route
     * @return array
     */
    protected function getRouteInformation(Route $route)
    {}

    /**
     * Sort the routes by a given element.
     *
     * @param  string  $sort
     * @param  array  $routes
     * @return array
     */
    protected function sortRoutes($sort, array $routes)
    {}

    /**
     * Remove unnecessary columns from the routes.
     *
     * @param  array  $routes
     * @return array
     */
    protected function pluckColumns(array $routes)
    {}

    /**
     * Display the route information on the console.
     *
     * @param  array  $routes
     * @return void
     */
    protected function displayRoutes(array $routes)
    {}

    /**
     * Get the URI for the given route, including any binding fields.
     *
     * @param  \Illuminate\Routing\Route  $route
     * @return string
     */
    protected function resolveUri(Route $route)
    {}

    /**
     * Get the middleware for the route.
     *
     * @param  \Illuminate\Routing\Route  $route
     * @return string
     */
    protected function getMiddleware($route)
    {}

    /**
     * Get the file path and line number for a closure-based route.
     *
     * @param  \Illuminate\Routing\Route  $route
     * @return string|null
     *
     * @throws \ReflectionException
     */
    protected function getClosurePath(Route $route)
    {}

    /**
     * Determine if the route has been defined outside of the application.
     *
     * @param  \Illuminate\Routing\Route  $route
     * @return bool
     */
    protected function isVendorRoute(Route $route)
    {}

    /**
     * Determine if the route uses a framework controller.
     *
     * @param  \Illuminate\Routing\Route  $route
     * @return bool
     */
    protected function isFrameworkController(Route $route)
    {}

    /**
     * Filter the route by URI and / or name.
     *
     * @param  array  $route
     * @return array|null
     */
    protected function filterRoute(array $route)
    {}

    /**
     * Get the table headers for the visible columns.
     *
     * @return array
     */
    protected function getHeaders()
    {}

    /**
     * Get the column names to show (lowercase table headers).
     *
     * @return array
     */
    protected function getColumns()
    {}

    /**
     * Parse the column list.
     *
     * @param  array  $columns
     * @return array
     */
    protected function parseColumns(array $columns)
    {}

    /**
     * Convert the given routes to JSON.
     *
     * @param  \Illuminate\Support\Collection  $routes
     * @return string
     */
    protected function asJson($routes)
    {}

    /**
     * Convert the given routes to regular CLI output.
     *
     * @param  \Illuminate\Support\Collection  $routes
     * @return array
     */
    protected function forCli($routes)
    {}

    /**
     * Get the formatted action for display on the CLI.
     *
     * @param  array  $route
     * @return string|null
     */
    protected function formatActionForCli($route)
    {}

    /**
     * Determine and return the output for displaying the number of routes in the CLI output.
     *
     * @param  \Illuminate\Support\Collection  $routes
     * @param  int  $terminalWidth
     * @return string
     */
    protected function determineRouteCountOutput($routes, $terminalWidth)
    {}

    /**
     * Get the terminal width.
     *
     * @return int
     */
    public static function getTerminalWidth()
    {}

    /**
     * Set a callback that should be used when resolving the terminal width.
     *
     * @param  \Closure|null  $resolver
     * @return void
     */
    public static function resolveTerminalWidthUsing($resolver)
    {}
}
