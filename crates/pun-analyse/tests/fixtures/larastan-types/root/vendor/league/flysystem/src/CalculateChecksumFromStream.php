<?php
declare(strict_types=1);

namespace League\Flysystem;

use function hash_final;
use function hash_init;
use function hash_update_stream;

trait CalculateChecksumFromStream
{
    private function calculateChecksumFromStream(string $path, Config $config): string
    {}

    /**
     * @return resource
     */
    abstract public function readStream(string $path);
}
