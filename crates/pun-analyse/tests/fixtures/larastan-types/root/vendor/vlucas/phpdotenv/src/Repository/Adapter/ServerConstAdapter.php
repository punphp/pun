<?php

declare(strict_types=1);

namespace Dotenv\Repository\Adapter;

use PhpOption\Option;
use PhpOption\Some;

final class ServerConstAdapter implements AdapterInterface
{
    /**
     * Create a new server const adapter instance.
     *
     * @return void
     */
    private function __construct()
    {}

    /**
     * Create a new instance of the adapter, if it is available.
     *
     * @return \PhpOption\Option<self>
     */
    public static function create()
    {}

    /**
     * Read an environment variable, if it exists.
     *
     * @param non-empty-string $name
     *
     * @return \PhpOption\Option<string>
     */
    public function read(string $name)
    {}

    /**
     * Write to an environment variable, if possible.
     *
     * @param non-empty-string $name
     * @param string           $value
     *
     * @return bool
     */
    public function write(string $name, string $value)
    {}

    /**
     * Delete an environment variable, if possible.
     *
     * @param non-empty-string $name
     *
     * @return bool
     */
    public function delete(string $name)
    {}
}
