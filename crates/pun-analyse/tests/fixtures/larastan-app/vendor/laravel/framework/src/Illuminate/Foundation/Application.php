<?php

namespace Illuminate\Foundation;

/**
 * Laravel's Application, reduced to what Larastan reads of it: the version and the core container aliases.
 */
class Application implements \Illuminate\Contracts\Foundation\Application
{
    /**
     * The Laravel framework version.
     *
     * @var string
     */
    const VERSION = '13.23.0';

    /**
     * Determine if the application is in the production environment.
     *
     * @return bool
     */
    public function isProduction()
    {
        return true;
    }

    /**
     * Get or check the current application environment.
     *
     * @param  string|array  ...$environments
     * @return string|bool
     */
    public function environment(...$environments)
    {
        return 'production';
    }

    /**
     * Register the core class aliases in the container.
     *
     * @return void
     */
    public function registerCoreContainerAliases()
    {
        foreach ([
            'app' => [self::class, \Illuminate\Contracts\Container\Container::class, \Illuminate\Contracts\Foundation\Application::class, \Psr\Container\ContainerInterface::class],
            'auth' => [\Illuminate\Auth\AuthManager::class, \Illuminate\Contracts\Auth\Factory::class],
            'auth.driver' => [\Illuminate\Contracts\Auth\Guard::class],
            'filesystem.disk' => [\Illuminate\Contracts\Filesystem\Filesystem::class],
        ] as $key => $aliases) {
            foreach ($aliases as $alias) {
                $this->alias($key, $alias);
            }
        }
    }
}
