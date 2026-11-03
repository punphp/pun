<?php

namespace Illuminate\Foundation;

use Illuminate\Contracts\Foundation\MaintenanceMode;

class ArrayMaintenanceMode implements MaintenanceMode
{
    /**
     * Indicates if maintenance mode is currently active.
     *
     * @var bool
     */
    protected $active = false;

    /**
     * The payload provided when maintenance mode was activated.
     *
     * @var array
     */
    protected $payload = [];

    /**
     * Take the application down for maintenance.
     *
     * @param  array  $payload
     * @return void
     */
    public function activate(array $payload): void
    {}

    /**
     * Take the application out of maintenance.
     *
     * @return void
     */
    public function deactivate(): void
    {}

    /**
     * Determine if the application is currently down for maintenance.
     *
     * @return bool
     */
    public function active(): bool
    {}

    /**
     * Get the data array which was provided when the application was placed into maintenance.
     *
     * @return array
     */
    public function data(): array
    {}
}
