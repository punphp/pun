<?php

namespace Illuminate\Log;

use InvalidArgumentException;
use Monolog\Level;

trait ParsesLogConfiguration
{
    /**
     * The Log levels.
     *
     * @var array
     */
    protected $levels = [
        'debug' => Level::Debug,
        'info' => Level::Info,
        'notice' => Level::Notice,
        'warning' => Level::Warning,
        'error' => Level::Error,
        'critical' => Level::Critical,
        'alert' => Level::Alert,
        'emergency' => Level::Emergency,
    ];

    /**
     * Get fallback log channel name.
     *
     * @return string
     */
    abstract protected function getFallbackChannelName();

    /**
     * Parse the string level into a Monolog constant.
     *
     * @param  array  $config
     * @return int
     *
     * @throws \InvalidArgumentException
     */
    protected function level(array $config)
    {}

    /**
     * Parse the action level from the given configuration.
     *
     * @param  array  $config
     * @return int
     *
     * @throws \InvalidArgumentException
     */
    protected function actionLevel(array $config)
    {}

    /**
     * Extract the log channel from the given configuration.
     *
     * @param  array  $config
     * @return string
     */
    protected function parseChannel(array $config)
    {}
}
