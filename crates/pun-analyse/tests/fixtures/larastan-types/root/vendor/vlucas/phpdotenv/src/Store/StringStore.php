<?php

declare(strict_types=1);

namespace Dotenv\Store;

final class StringStore implements StoreInterface
{
    /**
     * The file content.
     *
     * @var string
     */
    private $content;

    /**
     * Create a new string store instance.
     *
     * @param string $content
     *
     * @return void
     */
    public function __construct(string $content)
    {}

    /**
     * Read the content of the environment file(s).
     *
     * @return string
     */
    public function read()
    {}
}
