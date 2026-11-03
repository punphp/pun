<?php

namespace Illuminate\Foundation;

use Illuminate\Contracts\Foundation\MaintenanceMode as MaintenanceModeContract;

class FileBasedMaintenanceMode implements MaintenanceModeContract
{
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

    /**
     * Get the path where the file is stored that signals that the application is down for maintenance.
     *
     * @return string
     */
    protected function path(): string
    {}
}
