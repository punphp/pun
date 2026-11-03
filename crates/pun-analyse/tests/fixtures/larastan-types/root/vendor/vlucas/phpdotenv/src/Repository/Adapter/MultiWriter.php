<?php

declare(strict_types=1);

namespace Dotenv\Repository\Adapter;

final class MultiWriter implements WriterInterface
{
    /**
     * The set of writers to use.
     *
     * @var \Dotenv\Repository\Adapter\WriterInterface[]
     */
    private $writers;

    /**
     * Create a new multi-writer instance.
     *
     * @param \Dotenv\Repository\Adapter\WriterInterface[] $writers
     *
     * @return void
     */
    public function __construct(array $writers)
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
